use chain_of_responsibility::{patient::Patient, patient_funcs, Handler};

fn main() {
    // ハンドラを構築．

    // 支払い
    let pay_fee = Handler::new(patient_funcs::pay_fee).build();

    // 内視鏡
    let endoscopic = Handler::new(|patient| patient_funcs::endoscopic_imaging(patient, 10000))
        .next(pay_fee.clone())
        .build();

    // レントゲン
    let take_x_lay = Handler::new(|patient| patient_funcs::take_x_ray(patient, 5000))
        .next(pay_fee.clone())
        .failed_next(endoscopic) // 失敗したときは内視鏡検査
        .build();

    // 診療プロセス全体
    let medical_process = Handler::new(patient_funcs::medical_interview)
        .next(pay_fee)
        .failed_next(take_x_lay) // 失敗したときはレントゲン検査
        .build();

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
