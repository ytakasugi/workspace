use iced::{executor, Application, Command, Element, Text, Settings};

// ユニット構造体`GUI`を定義
struct GUI;

// icedクレートを使用する場合は、ApplicationかSandboxのどちらかを選択する必要があります。
//
// アプリケーションはサンドボックスよりも多くの機能を持っています。
// 先進的なアプリケーションの機能の一つにSubscriptionがあります。
// 非同期的な機能を実行することができます。
//
// ゲームやアニメのように一定間隔でウィンドウの内容を再描画したい場合は、サブスクリプションを利用してオリジナルの再描画イベントを作成する必要があります。
//
// そして、初期フラグとオプションを扱う必要がある場合は、アプリケーションを選択する必要があります。
///
// リッチなUIを必要とせず、非同期実行のないシンプルなUIが必要なだけなら、Sandboxを選ぶべきです。
//
// 今回の場合は、一定間隔でウィンドウの内容を再描画したいので、Applicationを使用します。

// GUI構造体に`Application`トレイトを実装する
impl Application for GUI {
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (GUI, Command::none())
    }

    // ウィンドウのタイトルを設定します。
    fn title(&self) -> String {
        String::from("DEMO")
    }

    // このメソッドは Message をイベントとして受け取ります。
    // また、メッセージによってアプリケーションの状態を変更します。
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    // このメソッドは、Windowに表示するウィジェットを返します。
    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, World!").into()
    }
}

fn main() {
    GUI::run(Settings::default());
}
