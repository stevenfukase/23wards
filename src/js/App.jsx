import React, { useState } from 'react';
// import wards from '../wards.json';
import { useWard } from './GlobalContext';
import Navbar from './components/Navbar';

export default function App() {
  const { dispatch, state } = useWard();
  const [loading, setLoading] = useState(false);

  const generateHandler = () => {
    setLoading(true);
    dispatch({ type: 'GENERATE' });
    setLoading(false);
  };

  return (
    <div className="h-screen">
      <Navbar />
      <div className="pt-16 h-full grid sm:grid-cols-2 divide-x">
        <div className="place-self-center transform sm:-translate-y-12 flex flex-col items-center">
          {!state && <h1>Welcome to Random Tokyo Ward generator</h1>}

          {state
            && (
              <>
                <h1 className="text-6xl">{state.ward}</h1>
                <h2 className="text-4xl">{state.japanese}</h2>
              </>
            )}

          {!loading
            && (
            <button
              type="button"
              className="focus:outline-none text-gray-500"
              onClick={generateHandler}
            >
              Generate Place
            </button>
            )}

        </div>
        <div>
          {loading && <h1>Loading...</h1>}
          {state
          && (
          <iframe
            title="map"
            className={`h-full w-full ${loading && 'hidden'}`}
            // className="h-full w-full"
            src={`https://www.google.com/maps/embed/v1/place?key=AIzaSyDjd3XyCKvPTWNeIKtEWJpUCDW874-XBvM&q=${encodeURIComponent(state.ward)}`}
            allowFullScreen
          />
          )}
        </div>
      </div>
    </div>
  );
}
