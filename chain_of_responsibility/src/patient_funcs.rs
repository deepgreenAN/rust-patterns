use crate::patient::Patient;
use crate::ProcessError;

pub fn medical_interview(mut patient: Patient) -> Result<Patient, ProcessError<Patient>> {
    patient
        .log_stack_mut()
        .push("問診を開始します．".to_string());

    *patient.fee_mut() += 500;

    if patient.level() < 10 {
        patient
            .log_stack_mut()
            .push("問診で解決しました．".to_string());
        Ok(patient)
    } else {
        Err(ProcessError { request: patient })
    }
}

pub fn take_x_ray(mut patient: Patient, x_ray_fee: u32) -> Result<Patient, ProcessError<Patient>> {
    patient
        .log_stack_mut()
        .push("レントゲンを撮影します．".to_string());

    *patient.fee_mut() += x_ray_fee;

    if patient.level() < 20 {
        patient
            .log_stack_mut()
            .push("レントゲンで解決しました．".to_string());
        Ok(patient)
    } else {
        Err(ProcessError { request: patient })
    }
}

pub fn endoscopic_imaging(
    mut patient: Patient,
    endoscopic_fee: u32,
) -> Result<Patient, ProcessError<Patient>> {
    patient
        .log_stack_mut()
        .push("内視鏡で撮影します．".to_string());

    *patient.fee_mut() += endoscopic_fee;

    if patient.level() < 30 {
        patient
            .log_stack_mut()
            .push("内視鏡で解決しました．".to_string());
        Ok(patient)
    } else {
        Err(ProcessError { request: patient })
    }
}

pub fn pay_fee(mut patient: Patient) -> Result<Patient, ProcessError<Patient>> {
    let fee = patient.fee();

    patient
        .log_stack_mut()
        .push(format!("診療代を{:?}円払いました．", fee));

    Ok(patient)
}
