
//  Rust 实现 状态模式   (State Pattern)


pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // ""

        // as_ref() 返回  Option<&Box<State>>
        // 如果不调用 as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数。
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state  = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State> ;

    // 如果参数有引用, 就要注意生命周期的声明
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }


    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Box::new(Published {})
        self
    }

}


struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}


struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


fn main() {

    // Draft --request_review()--> PendingReview --approve()--> Published


    let mut post = Post::new();

    post.add_text("I have a apple.");
    post.approve(); //对 Draft 调用 approve 没有任何内容
    post.request_review();
    println!("content: {}", post.content());

    post.request_review();
    println!("content: {}", post.content());


    post.approve();
    println!("content: {}", post.content());
}

