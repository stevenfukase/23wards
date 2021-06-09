import { useState } from 'react';
import '../styles/App.css';
import postalData from '../tokyo.json'

export default function App() {
  const [postalCode, setPostalCode] = useState();
  const [city, setCity] = useState();
  const [ward, setWard] = useState();
  const [prefecture, setPrefecture] = useState();

  const randomIntFromInterval = (min, max) => {
    return Math.floor(Math.random() * (max - min + 1) + min)
  }
  

  const generateRandomPlace = () => {
    const randInt = randomIntFromInterval(1, 1718);
    const { PostalCode, City, Ward, Prefecture } = postalData[randInt];
    setPostalCode(PostalCode);
    setCity(City);
    setWard(Ward);
    setPrefecture(Prefecture);

  }

  return (
    <div className="App">
      <h1>{postalCode && `${postalCode}, ${city}, ${ward}, ${prefecture}`}</h1>
      <button
        onClick={generateRandomPlace}
      >
        Generate Place
      </button>
    </div>
  );
}
