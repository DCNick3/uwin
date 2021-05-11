
import requests

from zipfile import ZipFile
from io import BytesIO

# loosely based on wine's make_unicode

MSDATA_LINK = "https://download.microsoft.com/download/C/F/7/CF713A5E-9FBC-4FD6-9246-275F65C0E498/Windows Supported Code Page Data Files.zip"

MSDATA_RESP = requests.get(MSDATA_LINK)
MSDATA_RESP.raise_for_status()
MSDATA = MSDATA_RESP.content
MSDATA_ZIPFILE = ZipFile(BytesIO(MSDATA))

CPs = [37, 437, 500, 708, 737, 775, 850, 852, 855, 857, 860, 861, 862, 863, 864, 865, 866, 869, 874, 875, 932, 936, 949, 950, 1026, 1250, 1251, 1252, 1253, 1254, 1255, 1256, 1257, 1258, 1361, 10000, 10001, 10002, 10003, 10004, 10005, 10006, 10007, 10008, 10010, 10017, 10021, 10029, 10079, 10081, 10082, 20127, 20866, 21866, 28591, 28592, 28593, 28594, 28595, 28596, 28597, 28598, 28599, 28603, 28605]

def get_cp_data(cp):
    bin_data = MSDATA_ZIPFILE.read(f"CodpageFiles/{cp:03}.txt")
    if bin_data[-1] == 0x1a:
        # remove the Substitute character
        # (see https://en.wikipedia.org/wiki/Substitute_character#End_of_file)
        bin_data = bin_data[:-2]
    return bin_data

#for cp in CPs:
#    print(cp)
#    print(get_cp_data(cp))

print("TODO: actually something useful")
