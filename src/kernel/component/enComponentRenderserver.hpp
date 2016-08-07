#ifndef EN_COMPONENT_RENDERSERVER_H
#define EN_COMPONENT_RENDERSERVER_H

#include "enComponentSpecialize.hpp"
#include "IRenderserver.hpp"
#include "Renderserver0.hpp"

class enComponentRenderserver0 : public enComponentSpecialize<IRenderserver, Renderserver0 > {
public:
    enComponentRenderserver0( Renderserver0 * instance ) : enComponentSpecialize<IRenderserver, Renderserver0 >( instance, enComponentType::RENDERSERVER ){}
};

#endif