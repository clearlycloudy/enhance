#ifndef LOGGER_STDOUT_H
#define LOGGER_STDOUT_H

#include <mutex>

#include "ILogger.hpp"

class LoggerStdout : public ILogger {
public:
    char const * get_id(){ return "LoggerStdout"; }
    void Log( char const * format, ... );
    void Log( std::string & );
private:
    std::mutex _mutex;
};

#endif
