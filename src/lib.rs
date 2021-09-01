#[macro_export]
macro_rules! map {
  {$($k: expr => $v: expr),* $(,)?} => {
    {
      let mut map = std::collections::HashMap::new();

      $(
        map.insert($k, $v);
      )*

      map
    }
  };
}

#[macro_export]
macro_rules! set {
  {$($v: expr),* $(,)?} => {
    {
      let mut set = std::collections::HashSet::new();

      $(
        set.insert($v);
      )*

      set
    }
  };
}
