use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Text,
};

fn main() {
    GUI::run(Settings::default());
}

// ユニット構造体`GUI`を定義
struct GUI {
    start_stop_button_state: button::State,
    reset_botton_state: button::State,
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
    type Message = ();
    type Flags = ();

    // `new()`メソッドは、`Application`トレイトを`run()`した際に、`iced`の内部で使用される初期化のために使用されるメソッド。
    // 自分で`new()`を実行する必要はない
    // 引数からフラグを得ることができるので、そのフラグに従って初期化処理を分岐することができる
    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI{
                // `start_stop_button_state`と`reset_botton_state`を初期化
                start_stop_button_state: button::State::new(),
                reset_botton_state: button::State::new()
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
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        );
    }
}