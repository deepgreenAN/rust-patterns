use chain_of_responsibility::{funcs, Handler, Patient};

fn main() {
    // ハンドラを構築．

    // 内視鏡
    let endoscopic = {
        let pay_fee = Handler::new(funcs::pay_fee).build();
        Handler::new(|patient| funcs::endoscopic_imaging(patient, 10000))
            .next(pay_fee)
            .build()
    };

    // レントゲン
    let take_x_lay = {
        let pay_fee = Handler::new(funcs::pay_fee).build();
        Handler::new(|patient| funcs::take_x_ray(patient, 5000))
            .next(pay_fee)
            .failed_next(endoscopic)
            .build()
    };

    // 診療プロセス全体
    let medical_process = {
        let pay_fee = Handler::new(funcs::pay_fee).build();
        Handler::new(funcs::medical_interview)
            .next(pay_fee)
            .failed_next(take_x_lay)
            .build()
    };

    // ハンドラを用いて実行
    let john = {
        let john = Patient::new("John".to_string(), 25);
        medical_process.execute(john)
    };

    println!("john: {:?}", john);

    let bob = {
        let bob = Patient::new("Bob".to_string(), 5);
        medical_process.execute(bob)
    };

    println!("bob: {:?}", bob);
}
