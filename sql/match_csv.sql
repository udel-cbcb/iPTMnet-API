CREATE OR REPLACE FUNCTION fn_match_csv (p_csv1         VARCHAR2,
                                           p_csv2         VARCHAR2,
                                          p_separator    VARCHAR2 DEFAULT ',')
    RETURN NUMBER
  IS
   l_cnt   NUMBER;
 BEGIN
 
     WITH tab1  -- convert p_csv1 into rows and resolve as a temp table tab1
         AS (SELECT REGEXP_SUBSTR (p_csv1,
                                    '[^' || p_separator || ']+',
                                    1,
                                    LEVEL)
                     data
                FROM dual
             CONNECT BY LEVEL <= LENGTH (p_csv1) - LENGTH (REPLACE (p_csv1, p_separator)) + 1),
          tab2  -- convert p_csv2 into rows and resolve as a temp table tab1
          AS (SELECT REGEXP_SUBSTR (p_csv2,
                                    '[^' || p_separator || ']+',
                                    1,
                                    LEVEL)
                     data
                FROM dual
          CONNECT BY LEVEL <= LENGTH (p_csv2) - LENGTH (REPLACE (p_csv2, p_separator)) + 1)
     SELECT COUNT (*) -- get count of matching values
       INTO l_cnt
       FROM tab1, tab2
      WHERE tab1.data = tab2.data
            AND tab1.data IS NOT NULL
            AND tab2.data IS NOT NULL;
 
      RETURN l_cnt;
 
   END;
