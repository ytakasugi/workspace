struct Years(i64);
struct Days(i64);

// `Years`にメソッドを実装
impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

// `Days`にメソッドを実装
impl Days {
    // 年齢の一部を切り捨てます
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();

    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));

    let years = Years(42);
    let years_as_primitive: i64 = years.0;

    println!("years {}", years_as_primitive);
}
