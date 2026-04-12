pub(crate) fn normalize_page(limit: Option<usize>, offset: Option<usize>) -> (usize, usize) {
    let limit = limit.unwrap_or(50).clamp(1, 200);
    let offset = offset.unwrap_or(0);
    (limit, offset)
}

pub(crate) fn paginate_vec<T>(items: Vec<T>, limit: usize, offset: usize) -> Vec<T> {
    items.into_iter().skip(offset).take(limit).collect()
}

pub(crate) fn now_ts() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}
