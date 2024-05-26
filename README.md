# Milky User Management Service

## Описание

**User**
- User creation
- User update
- User update self
- User get self
- User get by id
- User get by ids
- User range

**Session**
- Session creation (Sign in)
- Session deletion (Sign out)
- Session get by id
- Session get by user id

Структура хранения сессии:
...

**Role**
- Role creation
- Role update
- Role delete
- Role get by id
- Role range


Структура Role
- Идентификатор [Uuid] PK
- Имя Role [String]
- Описание Role [String]
- Список Permission [Uuid] MtM


**Permission**
- Permissions list

**Services**
- Идентификатор [Uuid] PK
- Идентификатор Service [String] PK
- Имя Service [String]

Структура Permission
- Идентификатор [Uuid] PK
- Service ForeingKey [Uuid] PK
- Текстовый индентификатор Permission [String] PK
- Имя Permission [String]
- CreatedAt [DateTime] - Дата создания для сортировки вывода

## JWT Payload for both tokens 
### [User -> API Gateway -> AuthService]
- User ID
- User State
- Binary Role as BIGINT as dec
- Session ID

## Header Payload 
### [API Gateway -> End Service]

- User ID
- User State
- PermissionsList

**PermissionsList**: {ServiceId: List[String]} - Список разрешений 
пользователя для каждого сервиса


UMS Слушает входящие запросы для подключения других сервисов, 
но в этом время он работает и обслужтвает клиентов


Каждый сервис при своем запуске сообщает UMS список своих permissions
И пока UMS не подтвердит их, сервис не должен принимать клиентские запросы


UMS хранит список permissions в РБД; В случае когда UNS получает
от сервиса список permissions, он проверяет их наличие в РБД и 
записывает их если они отсутствуют; 
UMS В случае появления новых permissions 
должен отправить всех пользователей в reauth-list TODO: оптимизировать что-то;
UMS должен обновить кэш TODO: какой-то кэш обсудить какие-то 
худшие последствия неправильного порядка permissions в бинарном представлении
и причем тут редис;


# Примечание

API gateway должен иметь в конфиге возможность включить проверку на 
содержание в куках и если в куках нет нужных полей то просто не 
ходить на сервис авторизации ???

При extract session если данные сессии (updated time) лежат более
n-минут - обновить данные в сессии;

Если время сессии создания прошло 30 дней то разлогаут

Защита проверка отпечатка браузера

В куках хранится не просто sessionId а sessionToken 
 

## Сборка

```bash
cargo build --release
```

## Migrations

Install `sea-orm-cli`:

```bash
cargo install sea-orm-cli
```

Setup the database URL:

```bash
export DATABASE_URL=protocol://username:password@localhost/database
```

Run the migrations:

```bash
sea-orm migration run
```
