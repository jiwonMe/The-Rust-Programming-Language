fn main() {
    // 일반적인 벡터 생성
    let v: Vec<i32> = Vec::new();
    // macro를 이용한 벡터 생성
    let v = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("vector:{:#?}", v);

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    println!("third:{:#?}", third);

    let v = vec![1, 2, 3, 4, 5];
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    println!("does_not_exist:{:#?}", does_not_exist);
}
