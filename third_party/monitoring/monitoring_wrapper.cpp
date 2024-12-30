#include <prometheus/counter.h>
#include <prometheus/exposer.h>
#include <prometheus/registry.h>
#include <memory>

using namespace prometheus;

class MonitoringWrapper {
private:
    std::shared_ptr<Registry> registry;
    Exposer exposer;

    Counter& file_encryption_counter;

public:
    MonitoringWrapper(const std::string& bind_address)
        : exposer(bind_address), registry(std::make_shared<Registry>()),
          file_encryption_counter(BuildCounter()
                                      .Name("file_encryption_requests_total")
                                      .Help("Total number of file encryption requests")
                                      .Register(*registry)) {
        exposer.RegisterCollectable(registry);
    }

    void incrementFileEncryptionCounter() {
        file_encryption_counter.Increment();
    }
};

// Example Usage
int main() {
    MonitoringWrapper monitoring("127.0.0.1:8081");
    monitoring.incrementFileEncryptionCounter();

    std::cout << "Prometheus metrics available at http://127.0.0.1:8081/metrics\n";
    return 0;
}
