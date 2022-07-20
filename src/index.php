<html>
<head>
<link href="style.css" rel="stylesheet">
<title>ShowRoom</title>
</head>
<body>
<div>
<h1>ShowRoom</h1>
<h2>Welcome to the ShowRoom</h2>
<?php
$servername = "localhost";
$username = "root";
$password = "Iegpiu12a!";
$dbname = "Weine";

$conn = new mysqli($servername, $username, $password, $dbname);
if($conn->connect_error){
    die("Connection failed: " . $conn->connect_error);
}
$sql = "SELECT * FROM Wein";
$result = $conn->query($sql);
if($result -> num_rows > 0){
    echo"<div class=`rowb`>Anzahl, Produzent, Jahrgang, Einkaufspreis, Verkaufspreis, Lieferung, Gesmantwert, Gesamtausgaben </div>";
    echo"<br>";
    while($row = $result->fetch_assoc()){
        if($row["Anzahl"]==0){
            echo "<div class='rowaus'><p>".$row["Anzahl"]." | ".$row["WeinName"]." | ".$row["Produzent"]." | ".$row["Jahrgang"]." | ".$row["Einkaufspreis"]." | ".$row["Verkaufspreis"]." | ".$row["Lieferung"]." | ".$row["id"]."</p></div>";
        }
        if($row["Anzahl"]>0){
            $Gesamtpreis = $row["Anzahl"]*$row["Verkaufspreis"];
            $Gesamrausgaben = $row["Anzahl"]*$row["Einkaufspreis"];
            echo "<div class='row'><p>".$row["Anzahl"]." | ".$row["WeinName"]." | ".$row["Produzent"]." | ".$row["Jahrgang"]." | ".$row["Einkaufspreis"]." | ".$row["Verkaufspreis"]." | ".$row["Lieferung"]." | ".$Gesamtpreis." | ".$Gesamrausgaben." | ".$row["id"]."</p></div>";
        }
        echo"<br>";
    }
}

?>
<center><button onclick="location.href='bearbeiten.php'">Bearbeiten</button></center>
<center><button onclick="location.href='erstellen.php'">Erstellen</button></center>
</div>
</body>
</html>