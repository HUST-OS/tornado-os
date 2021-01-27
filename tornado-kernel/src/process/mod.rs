use spin::Mutex;

pub struct Process {
    pub is_user: bool,
    pub inner: Mutex<ProcessInner>,    
}

pub struct ProcessInner {

}
