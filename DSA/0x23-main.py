#!/usr/bin/python3


def test_var_args(f_arg, *argv):
    print("first normal arg:", f_arg)
    for arg in argv:
        print("Anothe arg through *argv: ",arg)

test_var_args("test", "tesy", "Python3")



def sum_of_integers(n_arg, *argv):
    print("first sum is", n_arg + n_arg)
    for arg in argv:
        s = arg + arg 
        print("second sum is:", s + s)

sum_of_integers(56, 89, 90, 43, 23)



def add(*args):
    total = 0
    for arg in args:
        total += arg
    return total
print(add(1, 2, 3))



def display_name(**kwargs):
    for key, value in kwargs.items():
        print("{} : {}".format(key, value))

display_name(Student="Ian",Year="3", Course="Economics")



def shipping_label(*args, **kwargs):
    for arg in args:
        print(arg, end=" ")
    print()
    if 'apt' in kwargs:
        print(f"{kwargs.get('street')} {kwargs.get('apt')}")
    else:
        print(f"{kwargs.get('steet')}")
    print(f"{kwargs.get('city')} {kwargs.get('state')} {kwargs.get('code')}")
shipping_label("Dr", "Luke Odhiambo", "I", 
        street="123 Guilame",
        #apt = "100",
        city = "Detroit",
        state = "NRB",
        code = "0x023"
        )


def phone_number(**kwargs):
    for key, value in kwargs.items():
        print("name:{}: phone_number: {}".format(key, value))


phone_number(Moses = "254743279637", Jane="2545438990", Michael = "i547895423", Peter="254890433", Dana="3456789009", White="0x0456789")

