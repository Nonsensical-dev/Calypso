#ifndef CALYPSO_CLASS_H
#define CALYPSO_CLASS_H

// We don't need windows.h in this plugin but many others do and it throws up on itself all the time
// So best to include it and make sure CI warns us when we use something Microsoft took for their own goals....
#ifdef WIN32
#include <windows.h>
#endif

#include <godot_cpp/classes/ref.hpp>

using namespace godot;

class Calypso : public Object
{
    GDCLASS(Calypso, Object);

protected:
    static void _bind_methods();
private:
    static inline Calypso* singleton = nullptr;

    Calypso();
public:

    //Calypso(const Calypso&) = delete;
    //void operator=(const Calypso&) = delete;

    static Calypso* get_singleton();
    ~Calypso();
    void init();
};

#endif // CALYPSO_CLASS_H