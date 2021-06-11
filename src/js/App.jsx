import React, { useState } from 'react';
import wardData from '../wards.json';
import Navbar from './components/Navbar';

export default function App() {
  const [currentWardObj, setCurrentWardObj] = useState(null);
  const [isIframeOnload, setIsIframeOnload] = useState(true);
  const [isFirstVisit, setIsFirstVisit] = useState(true);
  // const [canBeSelectedWardId, setCanBeSelectedWardId] = useState();

  const randomIntFromInterval = (min, max) => Math.floor(Math.random() * (max - min + 1) + min);

  const generateRandomPlace = () => {
    if (isFirstVisit) {
      setIsFirstVisit(false);
    }
    setIsIframeOnload(true);
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

        <div className="place-self-center transform sm:-translate-y-12 flex flex-col items-center">
          {isFirstVisit && <h1>Welcome to Random Tokyo Ward generator</h1>}

          {!isIframeOnload && currentWardObj
            && (
              <div>
                <h1 className="text-6xl">{currentWardObj?.ward}</h1>
                <h2 className="text-4xl text-center">{currentWardObj?.japanese}</h2>
              </div>
            )}

          {!isIframeOnload && (
          <button
            type="button"
            className="focus:outline-none text-gray-500"
            onClick={generateRandomPlace}
          >
            Generate Place
          </button>
          )}

        </div>

        <div>
          {isIframeOnload ? <h1>loading...</h1> : null}
          <iframe
            title="map"
            className={`h-full w-full ${isIframeOnload && 'hidden'}`}
            onLoad={() => setIsIframeOnload(false)}
            src={`https://www.google.com/maps/embed/v1/place?key=AIzaSyDjd3XyCKvPTWNeIKtEWJpUCDW874-XBvM&q=${mapQuery}`}
            allowFullScreen
          />
        </div>

      </div>
    </div>
  );
}
