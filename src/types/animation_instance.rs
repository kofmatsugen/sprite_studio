use serde::*;

// インスタンスパーツの再生情報
#[derive(Deserialize, Serialize, Debug)]
pub struct InstanceKey {
    independent: bool, // 独立動作(true のときは新たにエンティティ生成する)
    #[serde(skip_serializing_if = "Option::is_none")]
    loop_num: Option<usize>, // ループ回数(None のときは無限)
    play_frame: usize, // 再生位置
    start_offset: usize, // 再生開始位置
    end_offset: usize, // 再生終了位置
    reverse: bool,     // 逆再生
    pingpong: bool,    // 往復再生
    speed_rate: f32,
}

impl InstanceKey {
    pub fn independent(&self) -> bool {
        self.independent
    }

    pub fn loop_num(&self) -> Option<usize> {
        self.loop_num
    }

    pub fn play_frame(&self) -> usize {
        self.play_frame
    }

    pub fn start_offset(&self) -> usize {
        self.start_offset
    }

    pub fn end_offset(&self) -> usize {
        self.end_offset
    }

    pub fn reverse(&self) -> bool {
        self.reverse
    }

    pub fn pingpong(&self) -> bool {
        self.pingpong
    }

    pub fn speed_rate(&self) -> f32 {
        self.speed_rate
    }
}

// インスタンスパーツ再生情報生成
#[derive(Default, Clone, Debug)]
pub struct InstanceKeyBuilder {
    infinity: Option<bool>,
    speed_rate: Option<f32>,
    independent: Option<bool>, // 独立動作(true のときは新たにエンティティ生成する)
    loop_num: Option<usize>,   // ループ回数(None のときは無限)
    play_frame: Option<usize>, // 再生フレーム
    start_offset: Option<usize>, // 再生開始位置
    end_offset: Option<usize>, // 再生終了位置
    reverse: Option<bool>,     // 逆再生
    pingpong: Option<bool>,    // 往復再生
}

impl InstanceKeyBuilder {
    pub fn infinity(mut self, infinity: bool) -> Self {
        self.infinity = infinity.into();
        self
    }

    pub fn independent(mut self, independent: bool) -> Self {
        self.independent = independent.into();
        self
    }

    pub fn loop_num(mut self, loop_num: usize) -> Self {
        self.loop_num = loop_num.into();
        self
    }

    pub fn play_frame(mut self, play_frame: usize) -> Self {
        self.play_frame = play_frame.into();
        self.start_offset = play_frame.into();
        self
    }

    pub fn end_offset(mut self, end_offset: usize) -> Self {
        self.end_offset = end_offset.into();
        self
    }
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse.into();
        self
    }
    pub fn pingpong(mut self, pingpong: bool) -> Self {
        self.pingpong = pingpong.into();
        self
    }

    pub fn speed_rate(mut self, speed_rate: f32) -> Self {
        self.speed_rate = speed_rate.into();
        self
    }

    // 現在のデータから次のキーを作成
    pub fn next_key(&self) -> Option<Self> {
        if let Some(true) = self.independent {
            // 独立再生の場合は続かない
            None
        } else {
            InstanceKeyBuilder {
                infinity: self.infinity,
                speed_rate: self.speed_rate,
                independent: self.independent, // 独立動作(true のときは新たにエンティティ生成する)
                loop_num: self.loop_num,       // ループ回数(None のときは無限)
                play_frame: self.play_frame.map(|frame| frame + 1), // 再生開始位置
                start_offset: self.start_offset,
                end_offset: self.end_offset, // 再生終了位置
                reverse: self.reverse,       // 逆再生
                pingpong: self.pingpong,     // 往復再生
            }
            .into()
        }
    }

    pub fn build(self) -> InstanceKey {
        let loop_num = if let Some(true) = self.infinity {
            None
        } else {
            self.loop_num
        };
        InstanceKey {
            speed_rate: self.speed_rate.unwrap_or(1.),
            independent: self.independent.unwrap_or(false), // 独立動作(true のときは新たにエンティティ生成する)
            loop_num,                                       // ループ回数(None のときは無限)
            play_frame: self.play_frame.unwrap_or(0),       // 再生開始位置
            start_offset: self.start_offset.unwrap_or(0),
            end_offset: self.end_offset.unwrap_or(0), // 再生終了位置
            reverse: self.reverse.unwrap_or(false),   // 逆再生
            pingpong: self.reverse.unwrap_or(false),  // 往復再生
        }
    }
}
