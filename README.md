# Entwicklung einer Stammbaum-App in Rust

## Zusammenfassung der Anforderungen

Für die Entwicklung einer Stammbaum-App in Rust sind folgende Anforderungen zu erfüllen:

- Erfassung von Personendaten:
    - Erfassung von Personendaten über eine benutzerfreundliche Webseite, einschließlich:
        - Name
        - Vorname
        - Geburtsdatum
        - Geschlecht
        - Adresse
        - Familienstand
        - Verknüpfungen zu Kindern und Eltern
    - Eine Person sollte über eine eindeutigen ID aus Name, Vorname und Geburtsdatum identifizierbar sein.

- Erfassung von Adressdaten:
  - Straßenname
  - Hausnummer
  - Postleitzahl
  - Stadt
  - Bundesland
  - Land

- Familienstand:
  - Mögliche Familienstände:
    - Ledig
    - Verheiratet
    - Geschieden
    - Verwitwet
  - Falls eine Person nicht ledig ist, sollte es möglich sein, die Person, mit der sie in einer Beziehung steht, über den Familienstand identifizierbar zu machen.

- Generierung des Stammbaums:
  - Der Stammbaum sollte visuell dargestellt werden können.
  - Exportmöglichkeiten für den generierten Stammbaum bzw. Graphen in den Formaten .dot, .png, .json, .pdf, .jpg, .jpeg sollten vorhanden sein.
  - Es sollte möglich sein, einen Stammbaum aus einer JSON-Datei zu importieren und darzustellen.
  - Änderungen am Stammbaum sollten über die graphische Benutzeroberfläche bzw. Webseite vorgenommen werden können.

- Visuelle Darstellung des Stammbaums


