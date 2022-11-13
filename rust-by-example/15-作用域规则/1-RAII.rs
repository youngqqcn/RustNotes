// Author: yqq
// Date: 2022-11-13 09:22:04
// Description: RAII

// RAII (Resource Acquisition Is Initializtion, 资源获取即初始化)
// 所有对象在离开作用域时，它的析构函数就被调用，资源释放
// 这种行为可以避免资源泄漏


fn create_box() {

    let _box1 = Box::new(2i32);
}

fn main() {

    let box1 = Box::new(333i32);

    {
        let _box2 = Box::new(4i32);
    }

    {
        for _ in 0..1000{
            create_box();
        }
    }
}