use askama::Template;
use bytestring::ByteString;
use futures_util::TryStreamExt;

#[derive(Template)]
#[template(path = "include.txt")]
struct Test {
    test_template_expr: Test2,
}

const fn get_template() -> Test2 {
    Test2
}

#[derive(Template)]
#[template(source = "2", ext = "txt", escape = "none")]
struct Test2;

#[tokio::test]
async fn include() {
    let vec: Vec<ByteString> = Test {
        test_template_expr: Test2,
    }
    .render()
    .try_collect()
    .await
    .unwrap();

    println!("{vec:?}");

    assert_eq!(
        vec,
        [
            ByteString::from_static("content\n"),
            ByteString::from_static("1"),
            ByteString::from_static("2"),
            ByteString::from_static("\n\n"),
            ByteString::from_static("2")
        ]
    );
}
