# Caesar-Verschlüsselung

TODO: Erklärung Caesar

## Aufgabe

Erstelle eine Funktion `caesar` mit folgenden Parametern:

- `text`: Eingabetext, der ver- oder entschlüsselt werden soll
- `key`: Anzahl Positionen, um die verschoben werden soll
- `encode`: `true` -> verschlüsseln, `false` -> entschlüsseln

Zurückgegeben werden soll der verschlüsselte bzw. entschlüsselte Text.

Anmerkungen:

- Zeichen, die keine Buchstaben von A-Z sind, sollen unverändert bleiben.
- Für die Testfälle könnt ihr davon ausgehen, dass alle Buchstaben Großbuchstaben sind.
Es steht euch aber natürlich frei, Kleinbuchstaben ebenfalls zu verschlüsseln.

## Bonus: Vigenere-Verschlüsselung

TODO: Erklärung Vigenere

Erstelle eine Funktion `vigenere` mit folgenden Parametern:

- `text`: wie bei `caesar`
- `key`: Schlüsselwort, enthält nur Großbuchstaben von A-Z 
- `encode`: wie bei `caesar`

Zurückgegeben soll auch hier wieder der ver- bzw. entschlüsselte Text.

Anmerkungen:

- wie bei `caesar`
- Zeichen, die nicht verschlüsselt werden, sollen kein Schlüsselzeichen "verbrauchen".