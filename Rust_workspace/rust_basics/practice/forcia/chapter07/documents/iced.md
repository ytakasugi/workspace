### [Crate iced](https://docs.rs/iced/0.2.0/iced/)

  - Description
    Iced は、シンプルさと型の安全性に焦点を当てたクロスプラットフォームの GUI ライブラリです。Elm にインスパイアされています。

---

- Feature

  - シンプルで使いやすい、batteries-included API
  - タイプセーフ、リアクティブプログラミングモデル
  - クロスプラットフォーム対応
  - レスポンシブレイアウト
  - 内蔵ウィジェット（テキスト入力、スクロール可能などを含む)
  - カスタムウィジェットのサポート（独自のウィジェットを作成)
  - パフォーマンスメトリクスのデバッグオーバーレイ
  - 非同期アクションのファーストクラスのサポート (futuresを使用してください！)
  - 再利用可能な部分に分割されたモジュラーエコシステム
    - 既存のシステムとの統合を可能にするレンダラーに依存しないネイティブランタイム
    - Vulkan、Metal、DX11、DX12をサポートする内蔵レンダラー
    - ウインドウイングシェル
    - DOM を活用したウェブランタイム

  詳細は、[リポジトリ](https://github.com/hecrj/iced)と[Example](https://github.com/hecrj/iced/tree/0.2/examples)を確認してください！

---

  - Overview

    The Elm ArchitectureにインスパイアされたIcedは、ユーザーインターフェースを4つの異なるコンセプトに分割することを期待しています。

    - State -  アプリケーションの状態
    - Message - ユーザーとのインタラクションや、気になる意味のあるイベント
    - View logic - ユーザーインタラクションでメッセージを生成するウィジェットとして状態を表示する方法
    - Update logic - メッセージに反応して状態を更新する方法

    これがどのように動作するかを確認するために何かを作ることができます 2つのボタンを使ってインクリメントとデクリメントができるシンプルなカウンターが欲しいとしましょう。

    アプリケーションの状態をモデル化することから始めます。

    ~~~rust
    use iced::button;
    
    struct Counter {
        // The counter value
        value: i32,
    
        // The local state of the two buttons
        increment_button: button::State,
        decrement_button: button::State,
    }
    ~~~

    次に、カウンタのユーザーインタラクションを定義する必要があります。これらのインタラクションがメッセージです。

    ~~~rust
    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        IncrementPressed,
        DecrementPressed,
    }
    ~~~

    では、実際のカウンターをビューロジックにまとめて見せてみましょう。

    ~~~rust
    use iced::{Button, Column, Text};
    
    impl Counter {
        pub fn view(&mut self) -> Column<Message> {
            // We use a column: a simple vertical layout
            Column::new()
                .push(
                    // The increment button. We tell it to produce an
                    // `IncrementPressed` message when pressed
                    Button::new(&mut self.increment_button, Text::new("+"))
                        .on_press(Message::IncrementPressed),
                )
                .push(
                    // We show the value of the counter here
                    Text::new(self.value.to_string()).size(50),
                )
                .push(
                    // The decrement button. We tell it to produce a
                    // `DecrementPressed` message when pressed
                    Button::new(&mut self.decrement_button, Text::new("-"))
                        .on_press(Message::DecrementPressed),
                )
        }
    }
    ~~~

    最後に、生成されたメッセージに反応し、更新ロジックでそれに応じて状態を変更できるようにする必要があります。

    ~~~rust
    impl Counter {
        // ...
    
        pub fn update(&mut self, message: Message) {
            match message {
                Message::IncrementPressed => {
                    self.value += 1;
                }
                Message::DecrementPressed => {
                    self.value -= 1;
                }
            }
        }
    }
    ~~~

    そして、それが全てだ！ユーザーインターフェース全体を書きました `Iced`ができるようになりました

    1. ビューロジックの結果を取得し、そのウィジェットをレイアウトします。
    2. システムからのイベントを処理し、更新ロジック用のメッセージを生成します。
    3. 結果のユーザーインターフェースを描画します。

    ---

    - Usage

      アプリケーションとサンドボックスの特性は、上記のすべてのプロセスを合理化して、すぐに始めることができるはずです!

    ---

    ### [Iced::Application](https://docs.rs/iced/0.2.0/iced/trait.Application.html)

    - Description

      インタラクティブなクロスプラットフォームのアプリケーション。

      この特徴は、Icedのメインの入り口です。一度実装されれば、単に run を呼び出すだけで GUI アプリケーションを実行することができます。

      - ネイティブプラットフォーム上では、それ自身のウィンドウで実行されます。
      - ウェブ上では、ドキュメントの <title> と <body> を制御します。

      アプリケーションはいくつかのメソッドでCommandを返すことで非同期アクションを実行することができます。プログラムの中でバックグラウンドでの作業を行うつもりがない場合、サンドボックスの特徴はシンプルなインターフェイスを提供します。デバッグ機能を有効にしたアプリケーションを使用している場合、F12キーを押すことでデバッグビューを切り替えることができます。

      ---

    - Example

      [リポジトリ](https://github.com/hecrj/iced/tree/0.2/examples)には、`Application trait`を使用したサンプル集があります。

      - clock: Canvasウィジェットを使って時計を描き、その針で現在の時刻を表示するアプリケーションです。
      - `download_progress`: 100 MB のダミーファイルを非同期でダウンロードし、ダウンロードの進行状況を追跡する基本的なアプリケーションです。
      - `events`: 条件付きサブスクリプションを使って表示されるネイティブイベントのログ。
      - `game_of_life`:  `[John Horton Conway]`が考案した`「Game of Life」`のインタラクティブ版。
      - `pokedex`: `PokéAPI`を使ってランダムにポケデックスのエントリー(スプライト付き！)を表示するアプリケーション。
      - `solar_system`: `Canvas`ウィジェットを使って描かれた太陽系のアニメーション。
      - `stopwatch`:  スタート/ストップとリセットボタンが付いた時計で、時間を聞く方法を紹介しています。
      - `todos`: `TodoMV`にインスパイアされた`TODOS`トラッカー。

    ---

    - A Simple "Hello, World"

      ~~~rust
      use iced::{executor, Application, Command, Element, Settings, Text};
      
      pub fn main() -> iced::Result {
          Hello::run(Settings::default())
      }
      
      struct Hello;
      
      impl Application for Hello {
          type Executor = executor::Default;
          type Message = ();
          type Flags = ();
      
          fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
              (Hello, Command::none())
          }
      
          fn title(&self) -> String {
              String::from("A cool application")
          }
      
          fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
              Command::none()
          }
          
          fn view(&mut self) -> Element<Self::Message> {
              Text::new("Hello, world!").into()
          }
      }
      ~~~


