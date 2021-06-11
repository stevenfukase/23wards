import React, { useState } from 'react';
// import '../styles/App.css';
import wardData from '../wards.json';
import Navbar from './components/Navbar';

export default function App() {
  const [currentWardObj, setCurrentWardObj] = useState(null);
  // const [canBeSelectedWardId, setCanBeSelectedWardId] = useState();

  const randomIntFromInterval = (min, max) => Math.floor(Math.random() * (max - min + 1) + min);

  const generateRandomPlace = () => {
    const randInt = randomIntFromInterval(1, 23);
    setCurrentWardObj(wardData[randInt]);
  };

  let mapQuery = 'tokyo';
  if (currentWardObj?.ward != null) {
    mapQuery = encodeURIComponent(currentWardObj.ward);
  }

  return (
    <div className="h-screen">
      <Navbar />
      <div className="pt-16 h-full grid sm:grid-cols-2">

        <div className="place-self-center transform sm:-translate-y-12">
          {currentWardObj
            ? (
              <div>
                <h1 className="text-6xl">{currentWardObj?.ward}</h1>
                <h2 className="text-4xl">{currentWardObj?.japanese}</h2>
              </div>
            )
            : <h1>Welcome to Random Tokyo Generator!</h1>}
          <button
            type="button"
            className="focus:outline-none text-gray-500"
            onClick={generateRandomPlace}
          >
            Generate Place
          </button>
        </div>

        <iframe
          title="map"
          className="h-full w-full"
          src={`https://www.google.com/maps/embed/v1/place?key=AIzaSyDjd3XyCKvPTWNeIKtEWJpUCDW874-XBvM&q=${mapQuery}`}
          allowFullScreen
        />
      </div>
    </div>
  );
}
