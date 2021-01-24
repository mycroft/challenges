use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h
        .iter()
        .fold(BTreeMap::<char,i32>::new(), 
            |mut t, (c, v)|  {
                v
                    .iter()
                    .map(
                        |e|
                            t.insert(e.to_ascii_lowercase(), *c)
                    )
                    .count();
                t
            }
        )
}
