fn main() {
    use mediator::{
        station::SingleTrackStation,
        train::{FreightTrain, PassengerTrain},
    };

    let mut station = SingleTrackStation::new();

    let train_1 = PassengerTrain::new("Train 1".to_string());
    let train_2 = FreightTrain::new("Train 2".to_string());
    let train_3 = PassengerTrain::new("Train 3".to_string());

    station.accept_train(train_1).unwrap();
    station.accept_train(train_2).unwrap();

    station.depart_train("Train 1").unwrap();

    station.accept_train(train_3).unwrap();

    station.depart_train("Train 2").unwrap();
    station.depart_train("Train 3").unwrap();
}
