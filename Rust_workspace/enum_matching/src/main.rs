#[derive(Debug)]
enum Color {
    Black,
    White
}

#[derive(Debug)]
enum GoodsId {
    Cup = 10000,
    Mouse = 218,
    Bolt = 508
}

fn color(a: u8) -> Color {
    if a < 128 {
        Color::Black
    } else {
        Color::White
    }
}

fn cast_goods_from_id(id: usize) -> Option<GoodsId> {
    match id {
        10000 => Some(GoodsId::Cup),
        218   => Some(GoodsId::Mouse),
        508   => Some(GoodsId::Bolt),
        _     => None
    }
}

fn main() {
    println!("{:?}", color(25));
    println!("{:?}", color(225));
    println!("{:?}", cast_goods_from_id(10000));
    println!("{:?}", cast_goods_from_id(218));
    println!("{:?}", cast_goods_from_id(508));
    println!("{}", GoodsId::Cup as usize);
}
