pub fn main() {
    println!(
        "\n 简单枚举.enum IpAddrKind {{
    V4,
    V6,
}}
使用::来表示值 IpAddrKind::V4
"
    );

    println!("\n枚举关联值.每个枚举成员都可以关联不同的值");
    {
        println!(
            "enum Message {{
    Quit,
    Move {{ x: i32, y: i32 }},
    Write(String),
    ChangeColor(i32, i32, i32),
}}"
        );
    }

    println!("如果没有枚举关联值，需要定义4个不同的结构体来表示4种不同的状态，并且他们类型还不同，无法为他们定义处理不同类型的结构体方法");
    {
        println!(
            "let q = QuitMessage{{}};
foo(&q);
let m = MoveMessage {{ x: 0, y: 0 }};
foo(&m);
let w = WriteMessage(\"hello\".to_string());
foo(&w);
let c = ChangeColorMessage(1,2,3);
foo(&c);"
        );
        let q = QuitMessage {};
        foo(&q);
        let m = MoveMessage { x: 0, y: 0 };
        foo(&m);
        let w = WriteMessage("hello".to_string());
        foo(&w);
        let c = ChangeColorMessage(1, 2, 3);
        foo(&c);
    }

    println!("有枚举关联值，可以直接为枚举定义方法，在内部走模式匹配");
    {
        println!(
            "let q = Message::Quit {{}};
let m = Message::Move {{ x: 0, y: 0 }};
let w = Message::Write(\"write\".to_string());
let c = Message::ChangeColor(1, 2, 3);
q.say();
m.say();
w.say();
c.say();"
        );
        let q = Message::Quit {};
        let m = Message::Move { x: 0, y: 0 };
        let w = Message::Write("write".to_string());
        let c = Message::ChangeColor(1, 2, 3);
        q.say();
        m.say();
        w.say();
        c.say();
    }

    println!("\nOption 枚举对于空值的优势");
    {
        println!("一个值要么是控制，要么是某个值。 因为Option和T是不同的类型，编译器不允许一个肯定有效的值和可能为空的Option值操作。 比如 i8 和 Option<i8> 无法相加");
        println!("Option必须转化位T才可以和T操作。这在进行API设计时，如果允许传入空值，参数就应该设计为Option,类似于C#中的CanBeNull宏，不过从类型上支持，更加好用。如果\
        不允许为空值，可以直接为T类型");
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn say(&self) {
        match &self {
            Message::Quit => {
                println!("Message::Quit say");
            }
            Message::Move { x, y } => {
                println!("Message::Move say  x = {},y = {}", x, y);
            }
            Message::Write(s) => {
                println!("Message::Write say s = {}", s);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Message::ChangeColor say r = {},g = {},b = {}", r, g, b);
            }
        }
    }
}

// 类单元结构体
struct QuitMessage;

struct MoveMessage {
    x: i32,
    y: i32,
}

// 元组结构体
struct WriteMessage(String);

// 元组结构体
struct ChangeColorMessage(i32, i32, i32);

trait IMessage {
    fn say(&self);
}

impl IMessage for QuitMessage {
    fn say(&self) {
        println!("QuitMessage say");
    }
}

impl IMessage for MoveMessage {
    fn say(&self) {
        println!("MoveMessage say. self.x = {}", self.x);
    }
}

impl IMessage for WriteMessage {
    fn say(&self) {
        println!("WriteMessage say. self.0 = {}", self.0);
    }
}

impl IMessage for ChangeColorMessage {
    fn say(&self) {
        println!("ChangeColorMessage say. self.0 = {}", self.0);
    }
}

fn foo<T>(imsg: &T)
where
    T: IMessage,
{
    imsg.say()
}
