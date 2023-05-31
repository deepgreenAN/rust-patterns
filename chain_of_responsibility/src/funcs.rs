use crate::{Patient, ProcessError};

pub fn medical_interview(mut patient: Patient) -> Result<Patient, ProcessError> {
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
        Err(ProcessError {
            patient,
            error_content: "エラー:問診で解決しませんでした．".to_string(),
        })
    }
}

pub fn take_x_ray(mut patient: Patient, x_ray_fee: u32) -> Result<Patient, ProcessError> {
    patient
        .log_stack_mut()
        .push("レントゲンを撮影する．".to_string());

    *patient.fee_mut() += x_ray_fee;

    if patient.level() < 20 {
        patient
            .log_stack_mut()
            .push("レントゲンで解決しました．".to_string());
        Ok(patient)
    } else {
        Err(ProcessError {
            patient,
            error_content: "エラー:レントゲンで解決しませんでした．".to_string(),
        })
    }
}

pub fn endoscopic_imaging(
    mut patient: Patient,
    endoscopic_fee: u32,
) -> Result<Patient, ProcessError> {
    patient
        .log_stack_mut()
        .push("内視鏡で撮影する．".to_string());

    *patient.fee_mut() += endoscopic_fee;

    if patient.level() < 30 {
        patient
            .log_stack_mut()
            .push("内視鏡で解決しました．".to_string());
        Ok(patient)
    } else {
        Err(ProcessError {
            patient,
            error_content: "エラー:内視鏡で解決しませんでした．".to_string(),
        })
    }
}

pub fn pay_fee(mut patient: Patient) -> Result<Patient, ProcessError> {
    let fee = patient.fee();

    patient
        .log_stack_mut()
        .push(format!("診療代を{:?}円払いました．", fee));

    Ok(patient)
}
