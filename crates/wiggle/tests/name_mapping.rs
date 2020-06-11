wiggle::from_witx!({
    witx_literal: "
        (typename $errno
            ( enum u8
                    $success
                    $2big 
                    ;; $1_weird_trick
            )
        )
    ",
    ctx: DummyCtx,
    // DEV KTM
    names: {
        "2big" => TooBig,
        // "1_weird_trick" => OneWeirdTrick
    },
});
