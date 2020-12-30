struct Polygon {
    vertexes: Vec<(i32, i32)>,  // 頂点の座標
    #[allow(dead_code)]
    stroke_width: u8,           // 輪郭の太さ
    fill: (u8, u8, u8),         // 塗りつぶしのRGB色
}

// フィールド名と同じ名前の関数引数やローカル変数がある時は以下のような
// 省略形も使える（Rust 1.17以降）
fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
    let stroke_width = 1;
    let fill = (0, 0, 0);
    Polygon { vertexes, stroke_width, fill }
}

fn main() {
    // Polygon型の値を作り、変数triangleを束縛する
    let triangle = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 1,
    };

    // vertexes, fill, stroke_widthの値を出力
    for i in triangle.vertexes.iter() {
        println!("triangle.vertexes = {:?}", i);
    }
    println!("triangle.fill = {:?}", triangle.fill);
    println!("triangle.stroke_width = {:?}", triangle.stroke_width);

    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    // フィールド名でアクセス
    assert_eq!(triangle.vertexes[0], (0, 0));
    assert_eq!(triangle.vertexes.len(), 3);
    assert_eq!(triangle.fill, (255, 255, 255));

    // パターンマッチでアクセス。不要なフィールドは..で省略できる
    let Polygon { vertexes: quad_vx, .. } = quadrangle;
    assert_eq!(4, quad_vx.len());
    println!("quad_vx.length = {}", quad_vx.len());

    for j in quad_vx.iter() {
        println!("quad_vx = {:?}", j);
    }

    // :以降を省略すると、フィールドと同じ名前の変数が作られフィールド値に束縛される
    let Polygon { fill, .. } = quadrangle;
    assert_eq!((0, 0, 0), fill);
    println!("quadrangle.fill = {:?}", fill);

    // 構造体の値を変更するにはmutが必要
    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    assert_eq!(polygon.vertexes.len(), 2);
    polygon.vertexes.push((2, 8));
    assert_eq!(polygon.vertexes.len(), 3);

    let triangle1 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 5,
    };

    for k in triangle1.vertexes.iter() {
        println!("triangle1.vertexes = {:?}", k);
    }
    println!("triangle1.fill = {:?}", triangle1.fill);
    println!("triangle1.stroke_width = {:?}", triangle1.stroke_width);

    // triangle1を元にvertexesだけ異なる新しい値を作る
    let triangle2 = Polygon {
        vertexes: vec![(0, 0), (-3, 0), (-2, 2)],
        .. triangle1
    };

    for s in triangle2.vertexes.iter() {
        println!("triangle2.vertexes = {:?}", s);
    }
}