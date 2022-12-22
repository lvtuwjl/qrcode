// The registry provides an interface for service discovery
// and an abstraction over varying implementations
// {consul, etcd, zookeeper, ...}
// type Registry interface {
// Init(...Option) error
// Options() Options
// Register(*Service, ...RegisterOption) error
// Deregister(*Service, ...DeregisterOption) error
// GetService(string, ...GetOption) ([]*Service, error)
// ListServices(...ListOption) ([]*Service, error)
// Watch(...WatchOption) (Watcher, error)
// String() string
// }

/// 注册中心需要实现方法
pub trait Registry {
    fn register(&self) {}

    fn deregister(&self) {}

    fn get_service(&self) {}

    fn list_services(&self) {}

    fn watch(&self) {}
}
