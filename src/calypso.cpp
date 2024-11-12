#include "calypso.h"

#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/variant/utility_functions.hpp>

using namespace godot;

Calypso::Calypso()
{
    singleton = this;
}

Calypso::~Calypso()
{
    singleton = nullptr;
}

Calypso* Calypso::get_singleton()
{
    return singleton;
}

void Calypso::init()
{
    UtilityFunctions::print("init");
}

void Calypso::_bind_methods()
{
    ClassDB::bind_method(D_METHOD("init"), &Calypso::init);
}