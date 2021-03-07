use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Text,
};

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 400u32);
    GUI::run(settings);
}

// `include_bytes`マクロは、指定したファイルの内容を実行バイナリに添付します。
//  これをバイナリ配列として使えばいいのです。
const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

#[derive(Debug, Clone)]
pub enum Message {
    // 時間の測定を開始するメッセージ
    Start,
    // 時間の測定を停止するメッセージ
    Stop,
    // 測定した時間をリセットするメッセージ
    Reset,
}

// アプリケーションが測定中かどうか管理するための列挙型
pub enum TickState {
    Stopped,
    Ticking,
}

// 構造体`GUI`を定義
struct GUI {
    tick_state: TickState,
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

// icedクレートを使用する場合は、ApplicationかSandboxのどちらかを選択する必要があります。
//
// アプリケーションはサンドボックスよりも多くの機能を持っています。
// 先進的なアプリケーションの機能の一つにSubscriptionがあります。
// 非同期的な機能を実行することができます。
//
// ゲームやアニメのように一定間隔でウィンドウの内容を再描画したい場合は、サブスクリプションを利用してオリジナルの再描画イベントを作成する必要があります。
//
// そして、初期フラグとオプションを扱う必要がある場合は、アプリケーションを選択する必要があります。
//
// リッチなUIを必要とせず、非同期実行のないシンプルなUIが必要なだけなら、Sandboxを選ぶべきです。
//
// 今回の場合は、一定間隔でウィンドウの内容を再描画したいので、Applicationを使用します。

// GUI構造体に`Application`を実装する
impl Application for GUI {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();

    // `new()`メソッドは、`Application`トレイトを`run()`した際に、`iced`の内部で使用される初期化のために使用されるメソッド。
    // 自分で`new()`を実行する必要はない
    // 引数からフラグを得ることができるので、そのフラグに従って初期化処理を分岐することができる
    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI{
                tick_state: TickState::Stopped,
                // `start_stop_button_state`と`reset_botton_state`を初期化
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new()
            }, 
            Command::none(),
        )
    }

    // ウィンドウのタイトルを設定します。
    fn title(&self) -> String {
        String::from("Timer")
    }

    // このメソッドは Message をイベントとして受け取ります。
    // また、メッセージによってアプリケーションの状態を変更します。
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        // 非同期処理の実行をランタイム側へ依頼することができる仕組み
        // 初期化時点で行いたい別処理を`Command`を使用して非同期で実行できる
        Command::none()
    }

    // このメソッドは、Windowに表示するウィジェットを返します。
    fn view(&mut self) -> Element<Self::Message> {
        // ウィジェットの初期化
        let tick_text = Text::new("00:00:00.00").font(FONT).size(60);
        let start_stop_button = Button::new(
            &mut self.start_stop_button_state,
            Text::new("Start")
                // テキストの水平方向の配置を中央寄せにする
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80);

        Column::new()
            // 行に要素を追加
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    // 要素間の垂直方向の間隔を設定
                    .spacing(10)
            )
            // 要素間の垂直方向の間隔を設定
            .spacing(10)
            // 列のパディングを設定
            .padding(10)
            // カラムの幅を設定
            .width(Length::Fill)
            // カラムの高さを設定
            .height(Length::Fill)
            // カラムの内容の水平方向のアライメントを設定
            .align_items(Align::Center)
            .into()
    }
}