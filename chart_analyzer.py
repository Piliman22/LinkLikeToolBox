import json
from enum import Enum
from dataclasses import dataclass
from typing import List, Optional, Dict, Any
import os

class NoteTypes(Enum):
    """ノーツタイプの定義"""
    SINGLE = 0  # タップノーツ
    HOLD = 1    # ホールドノーツ
    FLICK = 2   # フリックノーツ
    TRACE = 3   # トレースノーツ

@dataclass
class Flags:
    """Flags値を解析するクラス"""
    note_type: NoteTypes
    r1: int  # 右境界位置 (開始)
    r2: int  # 右境界位置 (終了)
    l1: int  # 左境界位置 (開始)
    l2: int  # 左境界位置 (終了)
    
    @property
    def width(self) -> int:
        """ノーツの幅を計算"""
        return self.r1 - self.l1
    
    @property
    def position_range(self) -> str:
        """位置範囲を文字列で表現"""
        return f"{self.l1}-{self.r1}"

@dataclass
class Note:
    """ノーツ情報を格納するクラス"""
    timing: float
    holds: List[float]
    uid: int
    flags: Flags
    raw_flags: int
    
    @property
    def note_type_name(self) -> str:
        """ノーツタイプ名を取得"""
        type_names = {
            NoteTypes.SINGLE: "Single(Tap)",
            NoteTypes.HOLD: "Hold",
            NoteTypes.FLICK: "Flick", 
            NoteTypes.TRACE: "Trace"
        }
        return type_names.get(self.flags.note_type, f"Unknown({self.flags.note_type.value})")
    
    @property
    def has_holds(self) -> bool:
        """ホールド情報があるかチェック"""
        return len(self.holds) > 0

def get_flags(val: int) -> Flags:
    """32ビット整数からFlags情報を抽出"""
    note_type = NoteTypes(val & 0xF)
    r1 = (val >> 4) & 0x3F
    r2 = (val >> 10) & 0x3F
    l1 = (val >> 16) & 0x3F
    l2 = (val >> 22) & 0x3F
    
    return Flags(note_type, r1, r2, l1, l2)

class ChartAnalyzer:
    """譜面データを解析するクラス"""
    
    def __init__(self, json_file_path: str):
        self.file_path = json_file_path
        self.notes: List[Note] = []
        self.bpms: List[Dict[str, Any]] = []
        self.offset: float = 0.0
        self.beats: List[Dict[str, Any]] = []
        
    def load_chart(self) -> bool:
        """譜面ファイルを読み込み"""
        try:
            with open(self.file_path, 'r', encoding='utf-8') as file:
                data = json.load(file)
                
            self.bpms = data.get('Bpms', [])
            self.offset = data.get('Offset', 0.0)
            self.beats = data.get('Beats', [])
            
            # ノーツデータを解析
            notes_data = data.get('Notes', [])
            for note_data in notes_data:
                if isinstance(note_data, dict) and 'Flags' in note_data:
                    timing = float(note_data.get('just', 0.0))
                    holds = [float(h) for h in note_data.get('holds', [])]
                    uid = note_data.get('Uid', 0)
                    raw_flags = note_data['Flags']
                    flags = get_flags(raw_flags)
                    
                    note = Note(timing, holds, uid, flags, raw_flags)
                    self.notes.append(note)
                    
            return True
            
        except Exception as e:
            print(f"エラー: 譜面ファイルの読み込みに失敗しました - {e}")
            return False
    
    def analyze_note(self, note: Note) -> Dict[str, Any]:
        """個別ノーツの詳細分析"""
        return {
            'timing': note.timing,
            'uid': note.uid,
            'type': note.note_type_name,
            'raw_flags': f"0x{note.raw_flags:08X}",
            'position': note.flags.position_range,
            'width': note.flags.width,
            'has_holds': note.has_holds,
            'hold_count': len(note.holds),
            'flags_detail': {
                'note_type': note.flags.note_type.value,
                'l1': note.flags.l1,
                'r1': note.flags.r1,
                'l2': note.flags.l2,
                'r2': note.flags.r2
            }
        }
    
    def get_statistics(self) -> Dict[str, Any]:
        """譜面統計情報を生成"""
        if not self.notes:
            return {}
            
        type_counts = {note_type: 0 for note_type in NoteTypes}
        for note in self.notes:
            type_counts[note.flags.note_type] += 1
            
        total_notes = len(self.notes)
        duration = max(note.timing for note in self.notes) if self.notes else 0
        
        # 同時押しの検出
        simultaneous_notes = []
        timing_groups = {}
        for note in self.notes:
            timing_key = f"{note.timing:.6f}"
            if timing_key not in timing_groups:
                timing_groups[timing_key] = []
            timing_groups[timing_key].append(note)
        
        for timing, notes_group in timing_groups.items():
            if len(notes_group) > 1:
                simultaneous_notes.append({
                    'timing': float(timing),
                    'count': len(notes_group),
                    'notes': [note.uid for note in notes_group]
                })
        
        return {
            'total_notes': total_notes,
            'duration': duration,
            'note_type_counts': {
                'Single': type_counts[NoteTypes.SINGLE],
                'Hold': type_counts[NoteTypes.HOLD], 
                'Flick': type_counts[NoteTypes.FLICK],
                'Trace': type_counts[NoteTypes.TRACE]
            },
            'simultaneous_count': len(simultaneous_notes),
            'bpm_info': self.bpms,
            'beat_info': self.beats,
            'offset': self.offset
        }
    
    def print_analysis(self, max_notes: int = 50):
        """分析結果を表示"""
        if not self.notes:
            print("ノーツデータがありません")
            return
            
        print(f"=== 譜面分析結果: {os.path.basename(self.file_path)} ===\n")
        
        # 統計情報
        stats = self.get_statistics()
        print("【統計情報】")
        print(f"総ノーツ数: {stats['total_notes']}")
        print(f"楽曲長: {stats['duration']:.2f}秒")
        print(f"BPM: {stats['bpm_info'][0]['Bpm'] if stats['bpm_info'] else 'N/A'}")
        print(f"拍子: {stats['beat_info'][0]['Numerator']}/{stats['beat_info'][0]['Denominator'] if stats['beat_info'] else 'N/A'}")
        print(f"オフセット: {stats['offset']}")
        print()
        
        print("【ノーツタイプ別集計】")
        for note_type, count in stats['note_type_counts'].items():
            percentage = (count / stats['total_notes']) * 100 if stats['total_notes'] > 0 else 0
            print(f"{note_type}: {count}個 ({percentage:.1f}%)")
        print()
        
        print(f"【同時押し】: {stats['simultaneous_count']}箇所")
        print()
        
        # 個別ノーツ分析
        print(f"【ノーツ詳細】(最初の{max_notes}個)")
        print("Time\t\tUID\tType\t\tFlags\t\tPosition\tWidth")
        print("-" * 80)
        
        for i, note in enumerate(self.notes[:max_notes]):
            analysis = self.analyze_note(note)
            print(f"{analysis['timing']:.6f}\t{analysis['uid']}\t{analysis['type']:<12}\t"
                  f"{analysis['raw_flags']}\t{analysis['position']}\t{analysis['width']}")
            
            if analysis['has_holds']:
                print(f"\t\t\t\t-> Hold終了: {note.holds}")
        
        if len(self.notes) > max_notes:
            print(f"... (残り {len(self.notes) - max_notes} ノーツ)")
    
    def export_to_csv(self, output_path: str):
        """CSV形式で出力"""
        import csv
        
        with open(output_path, 'w', newline='', encoding='utf-8') as csvfile:
            fieldnames = ['timing', 'uid', 'type', 'raw_flags', 'hex_flags', 
                         'l1', 'r1', 'l2', 'r2', 'position', 'width', 'holds']
            writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
            
            writer.writeheader()
            for note in self.notes:
                analysis = self.analyze_note(note)
                writer.writerow({
                    'timing': note.timing,
                    'uid': note.uid,
                    'type': note.note_type_name,
                    'raw_flags': note.raw_flags,
                    'hex_flags': f"0x{note.raw_flags:08X}",
                    'l1': note.flags.l1,
                    'r1': note.flags.r1,
                    'l2': note.flags.l2,
                    'r2': note.flags.r2,
                    'position': note.flags.position_range,
                    'width': note.flags.width,
                    'holds': ';'.join(map(str, note.holds)) if note.holds else ''
                })

def main():
    """メイン処理"""
    # 譜面ファイルのパス
    chart_files = [
        r"example\rhythmgame_chart_103103_01.bytes.json",
        r"example\rhythmgame_chart_103103_02.bytes.json", 
        r"example\rhythmgame_chart_103103_03.bytes.json",
        r"example\rhythmgame_chart_103103_04.bytes.json"
    ]
    
    for chart_file in chart_files:
        if os.path.exists(chart_file):
            print(f"\n{'='*60}")
            analyzer = ChartAnalyzer(chart_file)
            
            if analyzer.load_chart():
                analyzer.print_analysis(max_notes=30)
                
                # CSV出力
                csv_output = chart_file.replace('.json', '_analysis.csv')
                analyzer.export_to_csv(csv_output)
                print(f"\nCSV出力: {csv_output}")
            else:
                print(f"ファイル読み込みエラー: {chart_file}")
        else:
            print(f"ファイルが見つかりません: {chart_file}")

if __name__ == "__main__":
    main()