// Author: yqq
// Date: 2022-11-12 22:52:56
// Description: 使用关联类型



struct Container(i32, i32);

trait Contains {

    // 定义方法需要用到的泛型类型
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {

    // 需要指明 A， B的具体类型
    type A = i32;
    type B = i32;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }

}


fn difference<C: Contains>(container: &C) -> i32{
    container.last() - container.first()
}


fn main() {

    let a  = 3;
    let b = 100;

    let container = Container(a, b);

    println!("{}, {}:{}", &a, &b, container.contains(&a, &b));

    println!("{}", container.first());
    println!("{}", container.last());

    println!("{}", difference(&container));


}