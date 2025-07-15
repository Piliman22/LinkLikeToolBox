import json
import os
from typing import Dict, List, Any, Optional, Tuple
from dataclasses import dataclass
from chart_analyzer import ChartAnalyzer, NoteTypes, get_flags

@dataclass
class USCNote:
    """USC形式のノーツ情報"""
    beat: float
    lane: float
    size: float
    note_type: str
    critical: bool = False
    trace: bool = False
    direction: Optional[str] = None
    timeScaleGroup: int = 0

@dataclass
class USCSlideConnection:
    """USCスライドの接続点情報"""
    beat: float
    lane: float
    size: float
    connection_type: str  # "start", "tick", "end"
    critical: bool = False
    judgeType: str = "normal"
    ease: str = "linear"
    timeScaleGroup: int = 0

class ChartToUSCConverter:
    """リズムゲーム譜面をUSC形式に変換するクラス"""
    
    def __init__(self):
        # 標準的なサイズマッピング
        self.standard_width_14 = 14  # rhythmgame_chartでの標準幅
        self.standard_usc_size = 1.5  # USCでの対応サイズ
        
        self.time_tolerance = 0.01  # 時間の許容誤差（秒）
        self.beat_tolerance = 1e-6
        
    def _convert_timing_to_beat(self, timing: float, bpm: float) -> float:
        beat_length = 60.0 / bpm
        beat_position = timing / beat_length
        return round(beat_position * 64.0) / 64.0
    
    def _beats_equal(self, beat1: float, beat2: float) -> bool:
        """2つのビート値が等しいかを許容誤差を考慮して判定"""
        return abs(beat1 - beat2) < self.beat_tolerance
    
    def _get_note_lane_and_size_from_flags(self, flags, use_l2_r2: bool = False) -> tuple[float, float]:
        """Flagsからレーン位置とサイズを取得 (L1/R1またはL2/R2を使用)"""
        if use_l2_r2:
            left_pos = flags.l2
            right_pos = flags.r2
        else:
            left_pos = flags.l1
            right_pos = flags.r1
        
        # 中央位置を計算 (60分割)
        center_pos_60 = (left_pos + right_pos) / 2.0
        
        usc_lane = center_pos_60 / 59.0 * 12.0 - 6.0
        
        # -6.0~+6.0の範囲内に収める
        usc_lane = max(-6.0, min(6.0, usc_lane))
        
        # 0.5刻みに丸める
        usc_lane = round(usc_lane * 2) / 2
        
        # サイズを計算: (R1 - L1) / 2 * 12
        width_60 = right_pos - left_pos
        if width_60 <= 0:
            usc_size = 0.5  # 最小サイズ
        else:
            usc_size = width_60 / 59.0 * 6.0
            usc_size = max(0.5, round(usc_size * 2) / 2)
        
        return usc_lane, usc_size
    
    def _determine_note_type_and_direction(self, note) -> tuple[str, bool, Optional[str]]:
        """ノーツタイプと方向を判定"""
        note_type = note.flags.note_type
        trace = False
        direction = None
        
        if note_type == NoteTypes.SINGLE:
            return "single", trace, direction
        elif note_type == NoteTypes.FLICK:
            return "single", trace, "up"
        elif note_type == NoteTypes.TRACE:
            return "single", True, direction
        elif note_type == NoteTypes.HOLD:
            return "hold", trace, direction
        
        return "single", trace, direction
    
    def _get_hold_flags_key(self, flags: int, use_end: bool = False) -> int:
        if use_end:
            return flags & 0xFC0FC00  # 終点用マスク
        else:
            return flags & 0x03F03F0  # 開始点用マスク
    
    def _find_previous_hold(self, notes: List, current_index: int, bpm: float) -> int:
        current_note = notes[current_index]
        current_beat = self._convert_timing_to_beat(current_note.timing, bpm)
        current_flags = current_note.raw_flags
        
        search_key = (self._get_hold_flags_key(current_flags, use_end=False) << 6, current_beat)
        
        # 他のノーツで終点がこの位置と一致するものを探す
        for i, note in enumerate(notes):
            if i == current_index or not note.has_holds:
                continue
            
            # 最後のhold時間での終点ビート
            end_time = float(note.holds[-1])
            end_beat = self._convert_timing_to_beat(end_time, bpm)
            end_flags_key = self._get_hold_flags_key(note.raw_flags, use_end=True)
            
            if end_flags_key == search_key[0] and self._beats_equal(end_beat, search_key[1]):
                return i
        
        return current_index
    
    def _find_next_hold(self, notes: List, current_index: int, bpm: float) -> int:
        current_note = notes[current_index]
        if not current_note.has_holds:
            return current_index
        
        # 現在のノーツの終点情報
        end_time = float(current_note.holds[-1])
        end_beat = self._convert_timing_to_beat(end_time, bpm)
        current_flags = current_note.raw_flags
        
        end_flags_key = (self._get_hold_flags_key(current_flags, use_end=True) >> 6, end_beat)
        
        # 他のノーツで開始点がこの終点と一致するものを探す
        for i, note in enumerate(notes):
            if i == current_index or not note.has_holds:
                continue
            
            start_beat = self._convert_timing_to_beat(note.timing, bpm)
            start_flags_key = self._get_hold_flags_key(note.raw_flags, use_end=False)
            
            if start_flags_key == end_flags_key[0] and self._beats_equal(start_beat, end_flags_key[1]):
                return i
        
        return current_index
    
    def _find_real_start(self, notes: List, start_index: int, bpm: float) -> int:
        current = start_index
        while True:
            previous = self._find_previous_hold(notes, current, bpm)
            if current == previous:
                break
            current = previous
        return current
    
    def _find_real_end(self, notes: List, start_index: int, bpm: float) -> int:
        current = start_index
        while True:
            next_hold = self._find_next_hold(notes, current, bpm)
            if current == next_hold:
                break
            current = next_hold
        return current
    
    def _find_connected_holds_packer_style(self, notes: List, start_index: int, bpm: float) -> List[int]:
        real_start = self._find_real_start(notes, start_index, bpm)
        current = real_start
        connected_indices = []
        
        # デバッグ出力
        # if start_index < 10:  # 最初の10個のみ詳細ログ
        #     print(f"DEBUG: ノーツ{start_index} -> real_start={real_start}")
        #     start_note = notes[start_index]
        #     real_start_note = notes[real_start]
        #     print(f"  start_flags=0x{start_note.raw_flags:08X}, real_start_flags=0x{real_start_note.raw_flags:08X}")
        
        while True:
            connected_indices.append(current)
            next_hold = self._find_next_hold(notes, current, bpm)
            
            if start_index < 10:
                current_note = notes[current]
                print(f"  current={current}, next={next_hold}, flags=0x{current_note.raw_flags:08X}")
                if current_note.has_holds:
                    end_time = float(current_note.holds[-1])
                    end_beat = self._convert_timing_to_beat(end_time, bpm)
                    end_key = self._get_hold_flags_key(current_note.raw_flags, use_end=True) >> 6
                    print(f"    end_beat={end_beat:.6f}, end_key=0x{end_key:08X}")
            
            if current == next_hold:
                break
            current = next_hold
        
        return connected_indices
    
    def _interpolate_lane_and_size(self, start_flags, end_flags, progress: float) -> tuple[float, float]:
        # 開始点の位置 (L1, R1)
        start_l = start_flags.l1 / 59.0
        start_r = start_flags.r1 / 59.0
        
        # 終了点の位置 (L2, R2) 
        end_l = start_flags.l2 / 59.0
        end_r = start_flags.r2 / 59.0
        
        # L = tickPercent * (L2 - L1) + L1
        # R = tickPercent * (R2 - R1) + R1
        interp_l = progress * (end_l - start_l) + start_l
        interp_r = progress * (end_r - start_r) + start_r
        
        center = (interp_l + interp_r) / 2.0
        width = interp_r - interp_l
        
        usc_lane = center * 12.0 - 6.0
        usc_size = width * 6.0
        
        # 制限と丸め
        usc_lane = max(-6.0, min(6.0, usc_lane))
        usc_lane = round(usc_lane * 2) / 2
        usc_size = max(0.5, round(usc_size * 2) / 2)
        
        return usc_lane, usc_size
    
    def _create_complex_slide_packer_style(self, notes: List, indices: List[int], bpm: float) -> Optional[Dict[str, Any]]:
        if not indices:
            return None
        
        connections = []
        
        # 最初のノーツから開始点を作成
        first_note = notes[indices[0]]
        start_beat = self._convert_timing_to_beat(first_note.timing, bpm)
        start_lane, start_size = self._get_note_lane_and_size_from_flags(first_note.flags, use_l2_r2=False)
        
        connections.append({
            "beat": round(start_beat * 1000) / 1000,
            "critical": False,
            "ease": "linear",
            "judgeType": "normal",
            "lane": start_lane,
            "size": start_size,
            "timeScaleGroup": 0,
            "type": "start"
        })
        
        # デバッグ出力
        # if indices[0] < 10:
        #     print(f"  スライド開始: beat={start_beat:.6f}, lane={start_lane}, size={start_size}")
        
        # 各ホールドノーツの処理
        for i, note_index in enumerate(indices):
            current_note = notes[note_index]
            
            # 各hold点を処理
            for j, hold_timing in enumerate(current_note.holds):
                tick_beat = self._convert_timing_to_beat(float(hold_timing), bpm)
                
                start_time = current_note.timing
                end_time = float(current_note.holds[-1])
                if end_time > start_time:
                    tick_progress = (float(hold_timing) - start_time) / (end_time - start_time)
                else:
                    tick_progress = 0.0
                
                tick_lane, tick_size = self._interpolate_lane_and_size(
                    current_note.flags, 
                    current_note.flags,
                    tick_progress
                )
                
                # 最後のノーツの最後のholdかどうか判定
                is_final_point = (i == len(indices) - 1 and j == len(current_note.holds) - 1)
                
                if is_final_point:
                    # 最終点
                    connections.append({
                        "beat": round(tick_beat * 1000) / 1000,
                        "critical": False,
                        "judgeType": "trace",
                        "lane": tick_lane,
                        "size": tick_size,
                        "timeScaleGroup": 0,
                        "type": "end"
                    })
                    
                    if indices[0] < 10:
                        print(f"  スライド終点: beat={tick_beat:.6f}, lane={tick_lane}, size={tick_size}")
                else:
                    # 中継点
                    connections.append({
                        "beat": round(tick_beat * 1000) / 1000,
                        "ease": "linear",
                        "lane": tick_lane,
                        "size": tick_size,
                        "timeScaleGroup": 0,
                        "type": "tick"
                    })
                    
                    if indices[0] < 10:
                        print(f"  スライド中継点: beat={tick_beat:.6f}, lane={tick_lane}, size={tick_size}")
        
        return {
            "connections": connections,
            "critical": False,
            "type": "slide"
        }
    
    def convert_chart_to_usc(self, chart_file_path: str, output_path: str) -> bool:
        """譜面ファイルをUSC形式に変換"""
        try:
            # 譜面データを読み込み
            analyzer = ChartAnalyzer(chart_file_path)
            if not analyzer.load_chart():
                print(f"譜面ファイルの読み込みに失敗: {chart_file_path}")
                return False
            
            # BPM情報を取得
            if not analyzer.bpms:
                print("BPM情報が見つかりません")
                return False
            
            bpm = analyzer.bpms[0]['Bpm']
            offset = analyzer.offset
            
            print(f"変換情報: BPM={bpm}, オフセット={offset}")
            
            # USC形式のオブジェクトリストを作成
            usc_objects = []
            
            # BPMオブジェクトを追加
            usc_objects.append({
                "beat": 0.0,
                "bpm": float(bpm),
                "type": "bpm"
            })
            
            # タイムスケールグループを追加
            usc_objects.append({
                "changes": [
                    {
                        "beat": 0.0,
                        "timeScale": 1.0
                    }
                ],
                "type": "timeScaleGroup"
            })
            
            # ノーツを変換
            converted_count = 0
            processed_indices = set()  # 処理済みのインデックスを追跡
            
            # 全ノーツを処理
            for i, note in enumerate(analyzer.notes):
                # 既に処理済みの場合はスキップ
                if i in processed_indices:
                    continue
                
                beat = self._convert_timing_to_beat(note.timing, bpm)
                lane, size = self._get_note_lane_and_size_from_flags(note.flags, use_l2_r2=False)
                note_type_str, trace, direction = self._determine_note_type_and_direction(note)
                
                if note_type_str == "hold":
                    connected_indices = self._find_connected_holds_packer_style(analyzer.notes, i, bpm)
                    
                    # デバッグ出力（最初の5個のみ）
                    if i < 5:
                        real_start = self._find_real_start(analyzer.notes, i, bpm)
                        real_end = self._find_real_end(analyzer.notes, i, bpm)
                        print(f"ホールドノーツ{i+1}: real_start={real_start}, real_end={real_end}, 連続数={len(connected_indices)}")
                        print(f"  連続インデックス: {connected_indices}")
                        
                        for idx, conn_idx in enumerate(connected_indices):
                            conn_note = analyzer.notes[conn_idx]
                            conn_lane, conn_size = self._get_note_lane_and_size_from_flags(conn_note.flags)
                            print(f"  接続{idx+1}: index={conn_idx}, lane={conn_lane}, size={conn_size}, flags=0x{conn_note.raw_flags:08X}")
                    
                    # スライドオブジェクトを作成
                    slide_obj = self._create_complex_slide_packer_style(analyzer.notes, connected_indices, bpm)
                    if slide_obj:
                        usc_objects.append(slide_obj)
                        converted_count += 1
                        if i < 5:
                            print(f"  → スライド変換: {len(slide_obj['connections'])}個の接続点")
                    
                    # 処理済みのインデックスをマーク
                    for idx in connected_indices:
                        processed_indices.add(idx)
                        
                else:
                    # 通常のノーツオブジェクトを作成
                    note_obj = {
                        "beat": round(beat * 1000) / 1000,
                        "critical": trace,
                        "lane": lane,
                        "size": size,
                        "timeScaleGroup": 0,
                        "trace": trace,
                        "type": note_type_str
                    }
                    
                    # フリックの場合は方向を追加
                    if direction:
                        note_obj["direction"] = direction
                    
                    usc_objects.append(note_obj)
                    converted_count += 1
                    
                    # デバッグ出力（最初の10個のみ）
                    if i < 10:
                        print(f"ノーツ{i+1}: {note.timing:.6f}秒 → {beat:.6f}拍, レーン{lane}, サイズ{size}, タイプ{note_type_str}")
            
            # ビートでソート
            usc_objects_sorted = []
            
            # BPMとタイムスケールを最初に追加
            for obj in usc_objects:
                if obj.get("type") in ["bpm", "timeScaleGroup"]:
                    usc_objects_sorted.append(obj)
            
            # ノーツオブジェクトをビート順でソート
            note_objects = [obj for obj in usc_objects if obj.get("type") not in ["bpm", "timeScaleGroup"]]
            note_objects.sort(key=lambda x: x.get("beat", 0.0) if "beat" in x else min(conn.get("beat", 0.0) for conn in x.get("connections", [{}])))
            
            usc_objects_sorted.extend(note_objects)
            
            # USC形式のJSONを作成
            usc_data = {
                "usc": {
                    "objects": usc_objects_sorted,
                    "offset": -offset  # USCではオフセットが負の値
                },
                "version": 2
            }
            
            # ファイルに保存
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(usc_data, f, indent=4, ensure_ascii=False)
            
            print(f"USC形式に変換完了: {output_path}")
            print(f"変換されたノーツ数: {converted_count}")
            print(f"BPM: {bpm}")
            print(f"オフセット: {offset}")
            
            # 統計情報を表示
            stats = analyzer.get_statistics()
            print(f"ノーツタイプ別:")
            for note_type, count in stats['note_type_counts'].items():
                print(f"  {note_type}: {count}個")
            
            return True
            
        except Exception as e:
            print(f"変換エラー: {e}")
            import traceback
            traceback.print_exc()
            return False
    
    def convert_multiple_charts(self, input_dir: str, output_dir: str) -> None:
        """複数の譜面ファイルを一括変換"""
        if not os.path.exists(output_dir):
            os.makedirs(output_dir)
        
        chart_files = [f for f in os.listdir(input_dir) if f.endswith('.json')]
        
        for chart_file in chart_files:
            input_path = os.path.join(input_dir, chart_file)
            output_filename = chart_file.replace('.bytes.json', '.usc')
            output_path = os.path.join(output_dir, output_filename)
            
            print(f"\n変換中: {chart_file}")
            self.convert_chart_to_usc(input_path, output_path)

def main():
    """メイン処理"""
    converter = ChartToUSCConverter()
    
    # chartsフォルダ内の全JSONファイルを変換
    charts_dir = "charts"
    output_dir = "output_usc"
    
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    # chartsディレクトリの存在確認
    if not os.path.exists(charts_dir):
        print(f"エラー: {charts_dir} ディレクトリが見つかりません")
        print("chartsフォルダを作成して譜面ファイルを配置してください")
        return
    
    # chartsフォルダ内のJSONファイルを検索
    chart_files = []
    for file in os.listdir(charts_dir):
        if file.endswith('.json'):
            chart_files.append(file)
    
    if not chart_files:
        print(f"{charts_dir} ディレクトリに .json ファイルが見つかりません")
        return
    
    print(f"=== 一括変換開始 ===")
    print(f"入力ディレクトリ: {charts_dir}")
    print(f"出力ディレクトリ: {output_dir}")
    print(f"対象ファイル数: {len(chart_files)}個")
    print("-" * 50)
    
    success_count = 0
    failed_files = []
    
    for i, chart_file in enumerate(chart_files, 1):
        input_path = os.path.join(charts_dir, chart_file)
        output_filename = chart_file.replace('.bytes.json', '.usc').replace('.json', '.usc')
        output_path = os.path.join(output_dir, output_filename)
        
        print(f"\n[{i}/{len(chart_files)}] 変換中: {chart_file}")
        
        try:
            success = converter.convert_chart_to_usc(input_path, output_path)
            if success:
                print(f"✓ 変換成功: {output_filename}")
                success_count += 1
            else:
                print(f"✗ 変換失敗: {chart_file}")
                failed_files.append(chart_file)
        except Exception as e:
            print(f"✗ 変換エラー: {chart_file} - {str(e)}")
            failed_files.append(chart_file)
    
    # 結果
    print("\n" + "=" * 50)
    print("=== 変換結果 ===")
    print(f"総ファイル数: {len(chart_files)}個")
    print(f"変換成功: {success_count}個")
    print(f"変換失敗: {len(failed_files)}個")
    
    if failed_files:
        print("\n失敗したファイル:")
        for failed_file in failed_files:
            print(f"  - {failed_file}")
    
    print(f"\n出力ディレクトリ: {output_dir}")
    print("変換処理が完了しました。")
    
    
if __name__ == "__main__":
    main()