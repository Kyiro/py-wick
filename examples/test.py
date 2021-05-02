from os import listdir
import py_wick

PATH = "D:\\Games\\Fortnite\\FortniteGame\\Content\\Paks"
AES = "F941D9809A67D9BD104273E3C649F4395B6B6A874D16515F404B50D6A9FFA5A4"

query = input("Query: ")

for FileName in filter(lambda i: i.endswith(".ucas"), listdir(PATH)):
    FileName = FileName.replace(".ucas", "")
    Path = "\\".join([PATH, FileName])
    try:
        print(FileName + " | " + py_wick.read_pak_key(Path))
        Extractor = py_wick.Extractor(Path, AES)
        for i in Extractor.get_file_list():
            if query in i and i.endswith(".uasset"):
                print(FileName + " | " + i)
                try:
                    data = py_wick.Package(Extractor.get_file(i)).get_data()
                    print(data)
                except Exception as e:
                    print("Error when trying to get data " + str(e))
    except Exception as e:
        print(str(e))