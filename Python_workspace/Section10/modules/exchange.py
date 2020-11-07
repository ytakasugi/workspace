def yen_to_dollar(yen,rate,charge = 0):
    dollar = yen / (rate + charge)
    return dollar

def dollar_to_yen(dollar,rate,charge = 0):
    yen = dollar * (rate - charge)
    return yen