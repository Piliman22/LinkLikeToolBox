# 譜面ファイルについて

譜面ファイル構造が興味深い形だったので、忘れないように書いときます

## 目次

1. [基本の形](#基本の形)
   - 譜面ファイルの基本構造
   - Notes配列の形式
   - BPM・オフセット・拍子情報

2. [Flags値のビット構造](#flags値のビット構造)
   - 32ビット整数による情報エンコーディング
   - ビットフィールドの詳細解説
   - 具体例での解析結果

3. [ノーツタイプ](#ノーツタイプ)
   - ゲーム内ノーツタイプ一覧
   - 各タイプの操作方法
   - ゲームプレイメカニクス

4. [ノーツ位置](#ノーツ位置)
   - 位置座標システム
   - レーン配置マッピング
   - 位置計算の実例

5. [タイミング](#タイミング)
   - タイミング管理システム
   - BPMと拍数の変換
   - タイミングパターン分析

6. [同時押し、ホールドノーツ](#同時押しホールドノーツ)
   - 複雑なノーツパターンの処理
   - 同時押し検出とパターン分類
   - ホールドノーツの詳細分析

7. [全体](#全体)
   - 完全な解析フロー
   - データ変換プロセス
   - 統計生成とパターン分析


## 基本の形

```py
    "Notes": [
        {
            "just": "1.636364",    # タイミング（秒）
            "holds": [],           # ホールド終了時間の配列
            "Uid": 1,             # ユニークID
            "Flags": 224          # ノーツ情報を32ビットで格納
        }
    ],
    "Bpms": [                     # BPM変更情報
        {
            "Bpm": 110.0,
            "Start": "0",
            "Length": "300"
        }
    ],
    "Offset": 0.0,               # タイミングオフセット
    "Beats": [                   # 拍子情報
        {
            "Numerator": 4,
            "Denominator": 4,
            "Start": "0",
            "Length": "300"
        }
    ]
```

## Flags値のビット構造

```py
# 32ビット整数による情報エンコーディング
def decode_flags_structure():
    """
    Flags値の32ビット構造:
    
    31-28 27-22 21-16 15-10  9-4   3-0
    [未使用][L2] [L1] [R2]  [R1] [Type]
    
    各フィールド:
    - Type (4ビット): ノーツタイプ (0-15)
    - R1 (6ビット): 右境界位置・開始 (0-63)
    - R2 (6ビット): 右境界位置・終了 (0-63) 
    - L1 (6ビット): 左境界位置・開始 (0-63)
    - L2 (6ビット): 左境界位置・終了 (0-63)
    """
    
    # 実際の解析関数
    def get_flags(val: int) -> dict:
        return {
            'note_type': val & 0xF,           # ビット0-3
            'r1': (val >> 4) & 0x3F,          # ビット4-9
            'r2': (val >> 10) & 0x3F,         # ビット10-15
            'l1': (val >> 16) & 0x3F,         # ビット16-21
            'l2': (val >> 22) & 0x3F,         # ビット22-27
        }
    
    return get_flags

# 具体例での解析
EXAMPLE_FLAGS = {
    224: {  # 0x000000E0
        'binary': '00000000000000000000000011100000',
        'type': 0,     # Single(Tap)
        'l1': 0, 'r1': 14, 'l2': 0, 'r2': 0,
        'position': '0-14 (width: 14)'
    },
    983504: {  # 0x000F0170
        'binary': '00000000000011110000000101110000',
        'type': 0,     # Single(Tap)
        'l1': 15, 'r1': 23, 'l2': 0, 'r2': 0,
        'position': '15-23 (width: 8)'
    },
    2950066: {  # 0x002D0132
        'binary': '00000000001011010000000100110010',
        'type': 2,     # Flick
        'l1': 45, 'r1': 19, 'l2': 0, 'r2': 1,
        'position': '45-19 (フリック)'
    }
}
```

## ノーツタイプ
```py
from enum import Enum

class NoteTypes(Enum):
    """ゲーム内ノーツタイプ"""
    SINGLE = 0  # タップノーツ - 基本的なタップ操作
    HOLD = 1    # ホールドノーツ - 長押し操作
    FLICK = 2   # フリックノーツ - スワイプ操作
    TRACE = 3   # トレースノーツ - 指で軌跡を描く操作

class GameplayMechanics:
    """各ノーツタイプの操作方法"""
    
    @staticmethod
    def single_note():
        """タップノーツ: 指定位置をタップ"""
        return "画面の指定位置を正確なタイミングでタップ"
    
    @staticmethod  
    def hold_note():
        """ホールドノーツ: 開始から終了まで長押し"""
        return "開始位置でタップ後、終了時間まで押し続ける"
    
    @staticmethod
    def flick_note():
        """フリックノーツ: 指定方向にスワイプ"""
        return "タップ後、指定方向に素早くスワイプ"
    
    @staticmethod
    def trace_note():
        """トレースノーツ: 軌跡に沿って指を移動"""
        return "開始点から終了点まで、指定された軌跡に沿って指を移動"
```

## ノーツ位置

```py
class PositionSystem:
    """ゲーム画面の位置座標システム"""
    
    POSITION_RANGE = (0, 59)  # 横位置の範囲
    LANE_COUNT = 4           # 基本レーン数
    
    @staticmethod
    def get_lane_mapping():
        """基本的なレーン配置"""
        return {
            'lane_0': (0, 14),    # 左端レーン
            'lane_1': (15, 29),   # 左中レーン  
            'lane_2': (30, 44),   # 右中レーン
            'lane_3': (45, 59),   # 右端レーン
        }
    
    @staticmethod
    def calculate_position_info(l1: int, r1: int, l2: int = 0, r2: int = 0):
        """位置情報の計算"""
        return {
            'start_position': (l1, r1),
            'end_position': (l2, r2) if l2 or r2 else None,
            'width': r1 - l1,
            'center': (l1 + r1) / 2,
            'movement': ((l2 + r2) / 2 - (l1 + r1) / 2) if (l2 or r2) else 0
        }

# 実際の位置計算例
POSITION_EXAMPLES = {
    'normal_tap': {
        'flags': 224,
        'l1': 0, 'r1': 14,
        'description': '左端レーン、幅14の通常タップ'
    },
    'wide_note': {
        'flags': 1234567,
        'l1': 10, 'r1': 50,
        'description': '複数レーンにまたがる幅広ノーツ'
    },
    'moving_trace': {
        'flags': 85887681,
        'l1': 5, 'r1': 15, 'l2': 45, 'r2': 55,
        'description': '左から右に移動するトレースノーツ'
    }
}
```

## タイミング
```py
class TimingSystem:
    """ゲームのタイミング管理システム"""
    
    def __init__(self, bpm: float = 110.0, offset: float = 0.0):
        self.bpm = bpm
        self.offset = offset
        self.beat_duration = 60.0 / bpm  # 1拍の秒数
    
    def beats_to_seconds(self, beats: float) -> float:
        """拍数を秒数に変換"""
        return (beats * self.beat_duration) + self.offset
    
    def seconds_to_beats(self, seconds: float) -> float:
        """秒数を拍数に変換"""
        return (seconds - self.offset) / self.beat_duration
    
    def analyze_timing_pattern(self, notes_timing: list) -> dict:
        """タイミングパターンの分析"""
        intervals = []
        for i in range(1, len(notes_timing)):
            intervals.append(notes_timing[i] - notes_timing[i-1])
        
        return {
            'total_duration': max(notes_timing) - min(notes_timing),
            'average_interval': sum(intervals) / len(intervals) if intervals else 0,
            'min_interval': min(intervals) if intervals else 0,
            'max_interval': max(intervals) if intervals else 0,
            'note_density': len(notes_timing) / (max(notes_timing) - min(notes_timing))
        }

# BPM 110での実際の計算例
TIMING_EXAMPLES = {
    'beat_duration': 60.0 / 110.0,  # = 0.545454秒
    'quarter_note': 0.545454,       # 4分音符
    'eighth_note': 0.272727,        # 8分音符
    'sixteenth_note': 0.136364,     # 16分音符
}
```

## 同時押し、ホールドノーツ

```py
class AdvancedNotePatterns:
    """複雑なノーツパターンの処理"""
    
    @staticmethod
    def detect_simultaneous_notes(notes: list) -> list:
        """同時押しノーツの検出"""
        timing_groups = {}
        
        for note in notes:
            timing_key = f"{note.timing:.6f}"
            if timing_key not in timing_groups:
                timing_groups[timing_key] = []
            timing_groups[timing_key].append(note)
        
        simultaneous = []
        for timing, group in timing_groups.items():
            if len(group) > 1:
                simultaneous.append({
                    'timing': float(timing),
                    'count': len(group),
                    'notes': group,
                    'pattern_type': AdvancedNotePatterns.classify_chord(group)
                })
        
        return simultaneous
    
    @staticmethod
    def classify_chord(notes: list) -> str:
        """同時押しパターンの分類"""
        if len(notes) == 2:
            return "Double"
        elif len(notes) == 3:
            return "Triple"  
        elif len(notes) == 4:
            return "Quad"
        else:
            return f"{len(notes)}-Chord"
    
    @staticmethod
    def analyze_hold_pattern(note) -> dict:
        """ホールドノーツの詳細分析"""
        if not note.holds:
            return {'type': 'instant', 'duration': 0}
        
        hold_duration = max(note.holds) - note.timing
        
        return {
            'type': 'hold',
            'start_time': note.timing,
            'end_times': note.holds,
            'duration': hold_duration,
            'segments': len(note.holds),
            'is_trace': note.flags.note_type == NoteTypes.TRACE
        }

# 実際のパターン例
PATTERN_EXAMPLES = {
    'simple_chord': [
        {'timing': 2.454546, 'flags': 983504},    # 同時押し1
        {'timing': 2.454546, 'flags': 1967027}    # 同時押し2
    ],
    'long_hold': {
        'start': 6.545456,
        'holds': [6.818184, 7.090912],
        'duration': 0.545456,
        'type': 'multi-segment'
    },
    'complex_trace': {
        'segments': 5,
        'total_duration': 1.2,
        'movement_pattern': 'left_to_right_curve'
    }
}
```

## 全体

```py
class ChartDataFlow:    
    def complete_analysis_flow(self, chart_file: str):
        # 1. ファイル読み込み
        chart_data = self.load_json(chart_file)
        
        # 2. 基本情報抽出
        metadata = {
            'bpm': chart_data.get('Bpms', [{}])[0].get('Bpm', 110),
            'time_signature': chart_data.get('Beats', [{}])[0],
            'offset': chart_data.get('Offset', 0.0)
        }
        
        # 3. ノーツデータ解析
        notes = []
        for note_data in chart_data.get('Notes', []):
            if 'Flags' in note_data:
                flags = self.decode_flags(note_data['Flags'])
                note = {
                    'timing': note_data.get('just', 0.0),
                    'holds': note_data.get('holds', []),
                    'uid': note_data.get('Uid', 0),
                    'type': flags['note_type'],
                    'position': (flags['l1'], flags['r1']),
                    'end_position': (flags['l2'], flags['r2']),
                    'raw_flags': note_data['Flags']
                }
                notes.append(note)
        
        # 4. パターン分析
        patterns = {
            'simultaneous': self.detect_simultaneous(notes),
            'holds': self.analyze_holds(notes),
            'traces': self.analyze_traces(notes),
            'difficulty_metrics': self.calculate_difficulty(notes)
        }
        
        # 5. 統計生成
        statistics = self.generate_statistics(notes, metadata)
        
        return {
            'metadata': metadata,
            'notes': notes,
            'patterns': patterns,
            'statistics': statistics
        }

# データ処理の各段階での変換例
DATA_TRANSFORMATION = {
    'raw_json': {'Flags': 224, 'just': '1.636364'},
    'decoded_flags': {'type': 0, 'l1': 0, 'r1': 14, 'l2': 0, 'r2': 0},
    'game_object': {'type': 'Single', 'position': '0-14', 'timing': 1.636364},
    'gameplay_instruction': {'action': 'tap', 'lane': 0, 'timing': 1.636364}
}
```