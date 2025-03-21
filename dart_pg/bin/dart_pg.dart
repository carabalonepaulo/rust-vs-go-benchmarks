import 'package:postgres/postgres.dart';

void main() async {
  final startTime = DateTime.now();

  final conn = await Connection.open(
    Endpoint(
        host: 'localhost',
        database: 'redepetdb',
        username: 'redepetuser',
        password: 'redepet123'),
    settings: ConnectionSettings(sslMode: SslMode.disable),
  );

  final result = await conn.execute('SELECT * FROM test_table');

  for (var row in result) {
    var id = row[0];
    var name = row[1];
    // print('Row: $id, $name');
  }

  final duration = DateTime.now().difference(startTime);
  print(
      'Tempo total: ${duration.inSeconds}.${(duration.inMilliseconds % 1000).toString().padLeft(3, '0')} segundos');

  await conn.close();
}
