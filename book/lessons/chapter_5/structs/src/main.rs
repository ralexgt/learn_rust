const BLACK: Color = Color(0, 0, 0);
const ORIGIN: Point = Point(0, 0, 0);

// definition of a struct
#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u32,
}

fn build_user(name: String, email: String) -> User {
    User {
        active: true,
        name,
        email,
        sign_in_count: 1,
    }
}

struct Color(u8, u8, u8);
struct Point(i8, i8, i8);

fn main() {
    // initialization of a struct
    let user1 = User {
        active: true,
        name: String::from("Alex"),
        email: "alex@email.com".to_string(),
        sign_in_count: 1,
    };
    println!("{:?}", user1);
    println!(
        "{{ {}, {}, {}, {}, }}",
        user1.active, user1.name, user1.email, user1.sign_in_count
    );

    let mut user2 = User {
        active: user1.active,
        name: user1.name,
        email: String::from("someother@email.com"),
        sign_in_count: 1,
    };
    user2.active = false;
    println!("{:?}", user2);

    let user3 = User {
        email: String::from("That's an email for real man"),
        ..user2
    };
    println!("{:?}", user3);

    let user_built = build_user(
        String::from("somename"),
        String::from("that's an email trust me bro"),
    );

    println!("{:?}", user_built);

    let Point(x, y, z) = ORIGIN;
    println!("{1} {2} {2}, {0}", x, y, z);
    let Color(r, g, b) = BLACK;
    println!("rgb({}, {}, {})", r, g, b);
}
