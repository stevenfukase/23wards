import React, { useState } from 'react';
import '../styles/App.css';
import postalData from '../tokyo.json';
import Navbar from './components/Navbar';

export default function App() {
  const [postalCode, setPostalCode] = useState('');
  const [city, setCity] = useState('');
  const [ward, setWard] = useState('');
  const [prefecture, setPrefecture] = useState('');
  const [historyArray, setHistoryArray] = useState([]);

  const randomIntFromInterval = (min, max) => Math.floor(Math.random() * (max - min + 1) + min);

  const generateRandomPlace = () => {
    if (postalCode) {
      setHistoryArray((prevState) => [...prevState, {
        postalCode, city, ward, prefecture,
      }]);
    }

    const randInt = randomIntFromInterval(1, 1718);
    const {
      PostalCode, City, Ward, Prefecture,
    } = postalData[randInt];
    const formattedPostalCode = `${PostalCode.substr(0, 3)}-${PostalCode.substr(3)}`;
    setPostalCode(formattedPostalCode);
    if (City !== 'IKANIKEISAIGANAIBAAI') {
      setCity(City);
    } else {
      setCity('');
    }
    setWard(Ward);
    setPrefecture(Prefecture);
  };

  return (
    <div className="App">
      <Navbar />
      <div className="pt-28">
        {postalCode
          ? (
            <div>
              <h1>{`〒${postalCode}`}</h1>
              <h2>{`${city ? `${city},` : ''} ${ward}, ${prefecture}`}</h2>
            </div>
          )
          : <h1>Welcome to Random Place in Tokyo Generator!</h1>}
        <button
          type="button"
          onClick={generateRandomPlace}
        >
          Generate Place
        </button>

        {historyArray[0] && <h3>History</h3>}
        {historyArray.map((items) => (
          <div>
            <p>{`〒${items.postalCode}`}</p>
            <p>{`${items.city ? `${items.city},` : ''} ${items.ward}, ${items.prefecture}`}</p>
          </div>
        ))}
      </div>
    </div>
  );
}