import React, { useState, useEffect } from 'react';
import styles from '../styles/ConfigModal.module.css';
import { invoke } from '@tauri-apps/api/tauri';

interface Config {
  api_url: string;
  api_key: string;
  model: string;
}

interface ConfigModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function ConfigModal({ isOpen, onClose }: ConfigModalProps) {
  const [config, setConfig] = useState<Config>({
    api_url: '',
    api_key: '',
    model: 'gpt-3.5-turbo',
  });

  useEffect(() => {
    if (isOpen) {
      invoke('get_config').then((savedConfig) => {
        setConfig(savedConfig as Config);
      });
    }
  }, [isOpen]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await invoke('set_config', { config });
      onClose();
    } catch (error) {
      console.error('Failed to save config:', error);
    }
  };

  if (!isOpen) return null;

  return (
    <div className={styles.modalOverlay}>
      <div className={styles.modalContent}>
        <h2 className={styles.title}>配置</h2>
        <form onSubmit={handleSubmit}>
          <div className={styles.inputGroup}>
            <label htmlFor="api_url">API URL:</label>
            <input
              id="api_url"
              type="text"
              className={styles.input}
              value={config.api_url}
              onChange={(e) => setConfig({ ...config, api_url: e.target.value })}
            />
          </div>
          <div className={styles.inputGroup}>
            <label htmlFor="api_key">API Key:</label>
            <input
              id="api_key"
              type="password"
              className={styles.input}
              value={config.api_key}
              onChange={(e) => setConfig({ ...config, api_key: e.target.value })}
            />
          </div>
          <div className={styles.inputGroup}>
            <label htmlFor="model">Model:</label>
            <input
              id="model"
              type="text"
              className={styles.input}
              value={config.model}
              onChange={(e) => setConfig({ ...config, model: e.target.value })}
            />
          </div>
          <div className={styles.buttonGroup}>
            <button type="submit" className={styles.saveButton}>保存</button>
            <button type="button" onClick={onClose} className={styles.cancelButton}>取消</button>
          </div>
        </form>
      </div>
    </div>
  );
}