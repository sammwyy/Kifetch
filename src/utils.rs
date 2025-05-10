use crate::modules::SystemInfo;

pub struct Size {
    pub bytes: f64,
    pub kb: f64,
    pub mb: f64,
    pub gb: f64,
    pub tb: f64,
    pub metric: String,
    pub metric_value: f64,
}

pub fn get_size(bytes: f64) -> Size {
    let kb = bytes / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;
    let tb = gb / 1024.0;

    let metric = if tb > 1.0 {
        "TB".to_string()
    } else if gb > 1.0 {
        "GB".to_string()
    } else if mb > 1.0 {
        "MB".to_string()
    } else if kb > 1.0 {
        "KB".to_string()
    } else {
        "B".to_string()
    };

    let metric_value = if tb > 1.0 {
        tb
    } else if gb > 1.0 {
        gb
    } else if mb > 1.0 {
        mb
    } else if kb > 1.0 {
        kb
    } else {
        bytes
    };

    Size {
        bytes,
        kb,
        mb,
        gb,
        tb,
        metric,
        metric_value,
    }
}

pub fn insert_size(
    prefix: &str,
    free_bytes: usize,
    used_bytes: usize,
    total_bytes: usize,
    info: &mut SystemInfo,
) {
    let free = get_size(free_bytes as f64);
    let used = get_size(used_bytes as f64);
    let total = get_size(total_bytes as f64);
    let percentage = if total.bytes > 0.0 {
        (used.bytes / total.bytes) * 100.0
    } else {
        0.0
    };
    let metric = total.metric;

    info.insert(format!("{}_metric", prefix), metric);
    info.insert(
        format!("{}_free", prefix),
        format!("{:.2}", free.metric_value),
    );
    info.insert(
        format!("{}_used", prefix),
        format!("{:.2}", used.metric_value),
    );
    info.insert(
        format!("{}_total", prefix),
        format!("{:.2}", total.metric_value),
    );
    info.insert(
        format!("{}_percentage", prefix),
        format!("{:.2}", percentage),
    );
}
