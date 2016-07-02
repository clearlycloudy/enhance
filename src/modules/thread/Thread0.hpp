#ifndef THREAD0_H
#define THREAD0_H

#include "IThread.hpp"

#include <atomic>
#include <thread>
#include <functional>

class Thread0 : public IThread {
public:
    Thread0() : _state(IThread::State::STOPPED), _task(nullptr) {}
    ~Thread0();
    State getstate() const;
    void setaction( IThread::Action );
    void settask( std::function<void(void)> );
private:
    std::atomic<IThread::State> _state;
    std::thread _thread;
    std::function<void(void)> _task;

    void runloop();
    void runtask();
};

#endif
