import React, { useState, useEffect } from 'react';
import styles from '../styles/Home.module.css';
import Head from 'next/head';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import ConfigModal from '../components/ConfigModal';

interface Config {
  api_url: string;
  api_key: string;
  model: string;
}

export default function Home() {
  const [sourceText, setSourceText] = useState('');
  const [translatedText, setTranslatedText] = useState('');
  const [sourceLang, setSourceLang] = useState('中文');
  const [targetLang, setTargetLang] = useState('英语');
  const [isTranslating, setIsTranslating] = useState(false);
  const [isConfigOpen, setIsConfigOpen] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');

  useEffect(() => {
    const unsubscribeProgress = listen('translation_progress', (event: any) => {
      console.log("Received translation progress:", event.payload);
      setTranslatedText(event.payload as string);
    });

    const unsubscribeError = listen('translation_error', (event: any) => {
      console.error("Received translation error:", event.payload);
      setErrorMessage(event.payload as string);
      setIsTranslating(false);
    });

    return () => {
      unsubscribeProgress.then(f => f());
      unsubscribeError.then(f => f());
    };
  }, []);

  const handleTranslate = async () => {
    setIsTranslating(true);
    setTranslatedText('');
    setErrorMessage('');

    try {
      console.log("Sending translation request");
      await invoke('translate', {
        request: {
          text: sourceText,
          source_lang: sourceLang,
          target_lang: targetLang,
        },
      });
      console.log("Translation request completed");
    } catch (error) {
      console.error('Translation error:', error);
      setErrorMessage(`翻译出错：${error}`);
      setTranslatedText('');
    } finally {
      setIsTranslating(false);
    }
  };

  const handleSwitchLanguages = () => {
    setSourceLang(targetLang);
    setTargetLang(sourceLang);
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleTranslate();
    }
  };

  return (
    <>
      <Head>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>翻译</title>
        <style jsx global>{`
          html, body {
            margin: 0;
            padding: 0;
            height: 100%;
            overflow: hidden;
          }
        `}</style>
      </Head>
      <div className={styles.container}>
        <div className={styles.translatorBox}>
          <div className={styles.header}>
            <h1>👽 Huzhou</h1>
            <button onClick={() => setIsConfigOpen(true)}>配置</button>
          </div>
          <div className={styles.languageSelector}>
            <select 
              value={sourceLang} 
              onChange={(e) => setSourceLang(e.target.value)}
              className={styles.languageSelect}
            >
              <option>中文</option>
              <option>英语</option>
              <option>日语</option>
            </select>
            <button className={styles.switchButton} onClick={handleSwitchLanguages}>
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M17 1l4 4-4 4"></path>
                <path d="M3 11V9a4 4 0 0 1 4-4h14"></path>
                <path d="M7 23l-4-4 4-4"></path>
                <path d="M21 13v2a4 4 0 0 1-4 4H3"></path>
              </svg>
            </button>
            <select 
              value={targetLang} 
              onChange={(e) => setTargetLang(e.target.value)}
              className={styles.languageSelect}
            >
              <option>英语</option>
              <option>中文</option>
              <option>日语</option>
            </select>
          </div>
          <textarea
            className={styles.textArea}
            placeholder="输入要翻译的文本"
            value={sourceText}
            onChange={(e) => setSourceText(e.target.value)}
            onKeyDown={handleKeyDown}
          />
          <button 
            className={styles.translateButton} 
            onClick={handleTranslate}
            disabled={isTranslating}
          >
            {isTranslating ? '翻译中...' : '翻译'}
          </button>
          {errorMessage && (
            <div className={styles.errorMessage}>
              {errorMessage}
            </div>
          )}
          <textarea
            className={styles.textArea}
            placeholder="翻译结果"
            value={translatedText}
            readOnly
          />
        </div>
      </div>
      <ConfigModal isOpen={isConfigOpen} onClose={() => setIsConfigOpen(false)} />
    </>
  );
}

// Remove the ConfigModal function from this file