use crate::{Patient, ProcessError};

type HandlerFuncBox = Box<dyn Fn(Patient) -> Result<Patient, ProcessError>>;

pub struct Handler {
    func: HandlerFuncBox,
    next: Option<Box<Handler>>,
    failed_next: Option<Box<Handler>>,
}

impl Handler {
    pub fn new<F: Fn(Patient) -> Result<Patient, ProcessError> + 'static>(
        func: F,
    ) -> HandlerBuilder {
        HandlerBuilder::new(func)
    }
    pub fn execute(&self, patient: Patient) -> Patient {
        // ハンドラ自体の関数を実行
        let patient_res = (self.func)(patient);

        match patient_res {
            Ok(mut patient) => {
                // 次のハンドラを持っている場合
                if let Some(next) = self.next.as_ref() {
                    next.execute(patient)
                } else {
                    patient
                        .log_stack_mut()
                        .push("プロセスが終了しました".to_string());
                    patient
                }
            }
            Err(e) => {
                let ProcessError {
                    mut patient,
                    error_content,
                } = e;

                // エラー文字列をログに加える．
                patient.log_stack_mut().push(error_content);

                // 失敗したときの次のハンドラを持っている場合
                if let Some(failed_next) = self.failed_next.as_ref() {
                    failed_next.execute(patient)
                } else {
                    patient
                        .log_stack_mut()
                        .push("プロセスが終了しました".to_string());
                    patient
                }
            }
        }
    }
}

pub struct HandlerBuilder {
    func: HandlerFuncBox,
    next: Option<Box<Handler>>,
    failed_next: Option<Box<Handler>>,
}

impl Default for HandlerBuilder {
    fn default() -> Self {
        Self {
            func: Box::new(|patient: Patient| Ok(patient)),
            next: None,
            failed_next: None,
        }
    }
}

impl HandlerBuilder {
    pub fn new<F: Fn(Patient) -> Result<Patient, ProcessError> + 'static>(
        func: F,
    ) -> HandlerBuilder {
        let func = Box::new(func) as HandlerFuncBox;

        HandlerBuilder {
            func,
            next: None,
            failed_next: None,
        }
    }
    pub fn next(&mut self, handler: Handler) -> &mut Self {
        self.next = Some(Box::new(handler));
        self
    }
    pub fn failed_next(&mut self, handler: Handler) -> &mut Self {
        self.failed_next = Some(Box::new(handler));
        self
    }
    pub fn build(&mut self) -> Handler {
        let HandlerBuilder {
            func,
            next,
            failed_next,
        } = std::mem::take(self);

        Handler {
            func,
            next,
            failed_next,
        }
    }
}
