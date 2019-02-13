use cool_thing::token::{Comment, CommentTextError};
use encoding_rs::{EUC_JP, UTF_8};

test_fixture!("Comment token", {
    test("Comment closing sequence in text", {
        parse_token!("<!-- foo -->", UTF_8, Comment, |c: &mut Comment<'_>| {
            let err = c
                .set_text("foo -- bar --> baz")
                .unwrap_err()
                .downcast_ref::<CommentTextError>()
                .cloned()
                .unwrap();

            assert_eq!(err, CommentTextError::CommentClosingSequence);
        });
    });

    test("Encoding-unmappable characters text", {
        parse_token!("<!-- foo -->", EUC_JP, Comment, |c: &mut Comment<'_>| {
            let err = c
                .set_text("foo\u{00F8}bar")
                .unwrap_err()
                .downcast_ref::<CommentTextError>()
                .cloned()
                .unwrap();

            assert_eq!(err, CommentTextError::UnencodableCharacter);
        });
    });

    test("Serialization", {
        serialization_test!(
            "<!-- foo -- bar -->",
            Comment,
            &[
                ("Parsed", Box::new(|_| {}), "<!-- foo -- bar -->"),
                (
                    "Modified text",
                    Box::new(|c| {
                        c.set_text("42 <!-").unwrap();
                    }),
                    "<!--42 <!--->",
                ),
                (
                    "With prepends and appends",
                    Box::new(|c| {
                        c.before("<div>Hey</div>");
                        c.before("<foo>");
                        c.after("</foo>");
                        c.after("<!-- 42 -->");
                    }),
                    "<div>Hey</div><foo><!-- foo -- bar --><!-- 42 --></foo>",
                ),
                (
                    "Removed",
                    Box::new(|c| {
                        c.remove();
                        c.before("<before>");
                        c.after("<after>");
                    }),
                    "<before><after>",
                ),
                (
                    "Replaced",
                    Box::new(|c| {
                        c.before("<before>");
                        c.after("<after>");
                        c.replace("<div></div>");
                        c.replace("<!--42-->");
                    }),
                    "<before><div></div><!--42--><after>",
                ),
            ]
        );
    });
});
