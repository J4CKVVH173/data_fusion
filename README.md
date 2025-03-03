# Data fusion

Библиотека позволяющая объединить набор файлов в единый поток байт. А потом разобрать его обратно.

## CLI

Запуск в двух режима

### Объединение набора файлов в один байтовый поток

```bash
data_fusion -f ./fixtures/f1.txt -f ./fixtures/f2.png
```

Выдаст файл с именем `fusion.raw`.

### Разбиение байтового потока

В работе
