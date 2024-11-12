#include "calypso.h"

#include <godot_cpp/core/class_db.hpp>

using namespace godot;

Calypso::Calypso()
{
}

Calypso::~Calypso()
{
}

void Calypso::init()
{
    
}





void Calypso::_bind_methods()
{
    ClassDB::bind_method(D_METHOD("init"), &Calypso::init);
}