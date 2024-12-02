use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let iter = (1..=10).into_iter();

    let off_thread_iter = iter.off_thread();

    for item in off_thread_iter {
        println!("Received: {}", item);
        thread::sleep(Duration::from_millis(1000));
    }
}

trait OffThreadExt: Iterator {
    /// 将这个迭代器转换为线程外迭代器：
    /// `next()` 调用发生在单独的工作线程
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
where
    T: Iterator + Send + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        let (sender, receiver) = mpsc::sync_channel(1024);
        thread::spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });
        receiver.into_iter()
    }
}
